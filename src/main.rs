#![allow(clippy::large_enum_variant)]
use interactive_clap::ToCliArgs;
pub use near_cli_rs::CliResult;
use near_cli_rs::Verbosity;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod common;
mod proposals;
mod staking;
mod types;
mod validators;

/// near-cli is a toolbox for interacting with NEAR protocol

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
struct Cmd {
    /// Quiet mode
    #[interactive_clap(long)]
    quiet: bool,
    /// TEACH-ME mode
    #[interactive_clap(long)]
    teach_me: bool,
    #[interactive_clap(subcommand)]
    command: self::Command,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(disable_back)]
/// What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter)
pub enum Command {
    #[strum_discriminants(strum(message = "validators   -   Lookup validators for given epoch"))]
    /// Lookup validators for given epoch
    Validators(self::validators::Validators),
    #[strum_discriminants(strum(
        message = "proposals    -   Show both new proposals in the current epoch as well as current validators who are implicitly proposing"
    ))]
    /// Show both new proposals in the current epoch as well as current validators who are implicitly proposing
    Proposals(self::proposals::Proposals),
    #[strum_discriminants(strum(
        message = "staking      -   For validators, there is an option to staking without deploying a staking pool smart contract (stake, unstake, view stake)"
    ))]
    /// For validators, there is an option to staking without deploying a staking pool smart contract (stake, unstake, view stake)
    Staking(self::staking::Staking),
}

fn main() -> CliResult {
    let config = near_cli_rs::config::Config::get_config_toml()?;

    #[cfg(not(debug_assertions))]
    let display_env_section = false;
    #[cfg(debug_assertions)]
    let display_env_section = true;
    color_eyre::config::HookBuilder::default()
        .display_env_section(display_env_section)
        .install()?;

    let cli = match Cmd::try_parse() {
        Ok(cli) => cli,
        Err(error) => error.exit(),
    };

    let verbosity = match (cli.quiet, cli.teach_me) {
        (true, _) => Verbosity::Quiet,
        (false, true) => Verbosity::TeachMe,
        (false, false) => Verbosity::Interactive,
    };
    let global_context = near_cli_rs::GlobalContext {
        config,
        offline: false,
        verbosity,
    };

    loop {
        match <Cmd as interactive_clap::FromCli>::from_cli(
            Some(cli.clone()),
            global_context.clone(),
        ) {
            interactive_clap::ResultFromCli::Ok(cli_cmd)
            | interactive_clap::ResultFromCli::Cancel(Some(cli_cmd)) => {
                eprintln!(
                    "Your console command:\n{} {}",
                    std::env::args().next().as_deref().unwrap_or("./validator"),
                    shell_words::join(cli_cmd.to_cli_args())
                );
                return Ok(());
            }
            interactive_clap::ResultFromCli::Cancel(None) => {
                eprintln!("Goodbye!");
                return Ok(());
            }
            interactive_clap::ResultFromCli::Back => {}
            interactive_clap::ResultFromCli::Err(optional_cli_cmd, err) => {
                if let Some(cli_cmd) = optional_cli_cmd {
                    eprintln!(
                        "Your console command:\n{} {}",
                        std::env::args().next().as_deref().unwrap_or("./validator"),
                        shell_words::join(cli_cmd.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
