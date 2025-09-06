use clap::Args as ClapArgs;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tracing::{debug, info};

use crate::error::{Error, Result};
use crate::html;
use crate::json_schema;

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Input JSON Schema file (defaults to stdin)
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Output file (defaults to stdout)
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Don't minify the output HTML
    #[arg(long = "no-minify")]
    pub no_minify: bool,
}

pub fn execute(args: Args) -> Result<()> {
    info!("Processing compilation to HTML.");

    // Read the schema
    let (schema_source, schema) = if let Some(input_path) = &args.input {
        // Read from file
        debug!("Reading schema from: {}", input_path.display());

        if !input_path.exists() {
            return Err(Error::Other(format!(
                "Input file '{}' not found.",
                input_path.display()
            )));
        }

        let content = fs::read_to_string(input_path).map_err(Error::Io)?;

        let schema: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            Error::Other(format!(
                "Failed to parse JSON from '{}': {}",
                input_path.display(),
                e
            ))
        })?;

        (format!("{}", input_path.display()), schema)
    } else {
        // Read from stdin
        debug!("No input file specified, reading from stdin.");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).map_err(Error::Io)?;

        let schema: serde_json::Value = serde_json::from_str(&buffer)?;
        ("stdin".to_string(), schema)
    };

    debug!("Generating HTML for: {}", schema_source);
    let html = json_schema::generate_html(&schema)?;

    // Minify by default (unless --no-minify is specified)
    let final_html = if !args.no_minify {
        debug!("Minifying HTML output.");
        html::minify(&html)
    } else {
        debug!("Skipping HTML minification.");
        html
    };

    // Write output
    write_output(&final_html, &args.output)?;

    info!("Successfully converted to HTML.");
    Ok(())
}

fn write_output(content: &str, output_path: &Option<PathBuf>) -> Result<()> {
    match output_path {
        Some(path) => {
            debug!("Writing output to: {}", path.display());
            fs::write(path, content).map_err(Error::Io)?;
        }
        None => {
            debug!("Writing output to stdout.");
            io::stdout()
                .write_all(content.as_bytes())
                .map_err(Error::Io)?;
        }
    }
    Ok(())
}
