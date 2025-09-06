use std::{
    cmp::min,
    env,
    fs::{create_dir_all, rename},
    io::{Cursor, ErrorKind},
    path::Path,
};

use clap::Args as ClapArgs;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::ClientBuilder;
use serde::Deserialize;
#[cfg(windows)]
use tracing::warn;
use tracing::{debug, info};

use crate::constants;
use crate::error::{Error, Result};

#[derive(Deserialize, Debug)]
struct GithubAssetResponse {
    browser_download_url: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct GithubResponse {
    tag_name: String,
    assets: Vec<GithubAssetResponse>,
}

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Version to upgrade to (defaults to latest)
    #[arg(long)]
    pub version: Option<String>,

    /// Force upgrade even if already on requested version
    #[arg(long)]
    pub force: bool,
}

pub fn execute(args: Args) -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;
    runtime.block_on(execute_async(args))
}

async fn execute_async(args: Args) -> Result<()> {
    info!("Checking for updates...");

    // Get cache directory for temporary download
    let cache_dir = get_cache_dir()?;
    create_dir_all(&cache_dir).map_err(|e| Error::Io(std::io::Error::other(e)))?;

    // Get release information from GitHub
    let release_info = get_release_info(&args).await?;
    let current_version = constants::APP_VERSION;

    // Check if update is needed
    if !args.force && release_info.tag_name.trim_start_matches('v') == current_version {
        info!("Already on the latest version ({})", current_version);
        return Ok(());
    }

    info!(
        "Upgrading from {} to {}",
        current_version,
        release_info.tag_name.trim_start_matches('v')
    );

    // Find the appropriate asset for this platform
    let asset = find_platform_asset(&release_info)?;

    // Download the update
    download_update(&cache_dir, asset).await?;

    // Apply the update
    apply_update(&cache_dir, &release_info)?;

    info!(
        "Successfully upgraded to version {}",
        release_info.tag_name.trim_start_matches('v')
    );

    Ok(())
}

fn get_cache_dir() -> Result<std::path::PathBuf> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        Ok(proj_dirs.cache_dir().to_path_buf())
    } else {
        // Fallback to temp directory
        Ok(std::env::temp_dir().join(format!("{}-update", env!("CARGO_PKG_NAME"))))
    }
}

async fn get_release_info(args: &Args) -> Result<GithubResponse> {
    let client = ClientBuilder::new()
        .build()
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;

    let url = if let Some(ref version) = args.version {
        format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            constants::GITHUB_OWNER,
            constants::GITHUB_REPO,
            version
        )
    } else {
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            constants::GITHUB_OWNER,
            constants::GITHUB_REPO
        )
    };

    debug!("Fetching release info from: {}", url);

    let resp = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;

    if !resp.status().is_success() {
        if resp.status() == 404 {
            return Err(Error::Other(if args.version.is_some() {
                format!("Release '{}' not found", args.version.as_ref().unwrap())
            } else {
                "No releases found for this project".to_string()
            }));
        }
        return Err(Error::Io(std::io::Error::other(format!(
            "GitHub API returned status: {}",
            resp.status()
        ))));
    }

    resp.json::<GithubResponse>()
        .await
        .map_err(|e| Error::Io(std::io::Error::other(e)))
}

fn find_platform_asset(release: &GithubResponse) -> Result<&GithubAssetResponse> {
    let target = env!("TARGET");

    let binary_name = constants::CLI_BIN;
    let expected_name = format!("{}-{}_{}.tar.gz", binary_name, release.tag_name, target);

    release
        .assets
        .iter()
        .find(|asset| asset.name == expected_name)
        .ok_or_else(|| {
            Error::Other(format!(
                "No pre-built binary found for target '{}' in release '{}'",
                target, release.tag_name
            ))
        })
}

async fn download_update(cache_dir: &Path, asset: &GithubAssetResponse) -> Result<()> {
    let client = ClientBuilder::new()
        .build()
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;

    info!("Downloading update from: {}", asset.browser_download_url);

    let res = client
        .get(&asset.browser_download_url)
        .send()
        .await
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;

    let total_size = res
        .content_length()
        .ok_or_else(|| Error::Io(std::io::Error::other("Failed to get content length")))?;

    // Set up progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb.set_message("Downloading update");

    // Download to memory buffer
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| Error::Io(std::io::Error::other(e)))?;
        buffer.extend_from_slice(&chunk);

        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_and_clear();
    info!("Download complete");

    // Extract the archive
    debug!("Extracting update archive");
    let mut cursor = Cursor::new(buffer);
    let gz = flate2::read::GzDecoder::new(&mut cursor);
    let mut archive = tar::Archive::new(gz);

    archive
        .unpack(cache_dir)
        .map_err(|e| Error::Io(std::io::Error::other(e)))?;

    Ok(())
}

fn apply_update(cache_dir: &Path, _release: &GithubResponse) -> Result<()> {
    let binary_name = constants::CLI_BIN;
    let update_binary = if cfg!(windows) {
        cache_dir.join(format!("{}.exe", binary_name))
    } else {
        cache_dir.join(binary_name)
    };

    if !update_binary.exists() {
        return Err(Error::Other(format!(
            "Downloaded binary not found at: {}",
            update_binary.display()
        )));
    }

    let current_exe = env::current_exe().map_err(|e| Error::Io(std::io::Error::other(e)))?;

    debug!(
        "Replacing {} with {}",
        current_exe.display(),
        update_binary.display()
    );

    // On Windows, we might need to rename the current executable first
    #[cfg(windows)]
    {
        let backup = current_exe.with_extension("old");
        if backup.exists() {
            std::fs::remove_file(&backup).ok();
        }
        if let Err(e) = rename(&current_exe, &backup) {
            warn!("Failed to create backup of current executable: {}", e);
        }
    }

    // Replace the current executable with the new one
    if let Err(e) = rename(&update_binary, &current_exe) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                return Err(Error::Other(format!(
                    "Permission denied: cannot write to {}. Try running with elevated privileges.",
                    current_exe.display()
                )));
            }
            _ => {
                return Err(Error::Io(e));
            }
        }
    }

    // Clean up Windows backup file
    #[cfg(windows)]
    {
        let backup = current_exe.with_extension("old");
        std::fs::remove_file(backup).ok();
    }

    Ok(())
}
