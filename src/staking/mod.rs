use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod stake_proposal;
mod unstake_proposal;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
pub struct Staking {
    #[interactive_clap(subcommand)]
    staking_command: StakingCommand,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter)
pub enum StakingCommand {
    #[strum_discriminants(strum(
        message = "stake-proposal      -   To stake NEAR directly without a staking pool"
    ))]
    /// To stake NEAR directly without a staking pool
    StakeProposal(self::stake_proposal::StakeProposal),
    #[strum_discriminants(strum(
        message = "unstake-proposal    -   To unstake NEAR directly without a staking pool"
    ))]
    /// To unstake NEAR directly without a staking pool
    UnstakeProposal(self::unstake_proposal::UnstakeProposal),
    #[strum_discriminants(strum(message = "current"))]
    /// Lookup validators for given epoch
    Current,
    #[strum_discriminants(strum(message = "historical"))]
    /// Staking management (proposal, current, historical)
    Historical,
}
