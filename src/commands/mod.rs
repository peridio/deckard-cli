use clap::Subcommand;

pub mod convert;
pub mod upgrade;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Upgrade the CLI to a newer version
    #[command(visible_alias = "u")]
    Upgrade(upgrade::Args),

    /// Convert JSON Schema to HTML documentation
    #[command(visible_alias = "c")]
    Convert(convert::Args),
}
