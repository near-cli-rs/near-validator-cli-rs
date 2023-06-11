use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod block_id;
mod latest;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
pub struct Validators {
    #[interactive_clap(subcommand)]
    epoch_command: EpochCommand,
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
/// Choose Block ID
pub enum EpochCommand {
    #[strum_discriminants(strum(message = "View latest validators"))]
    /// Specify latest validators
    Latest(self::latest::Latest),
    // #[strum_discriminants(strum(
    //     message = "View validators by EpochId"
    // ))]
    // EpochId(self::view_command::ViewQueryRequest),
    #[strum_discriminants(strum(message = "View validators by BlockId"))]
    /// Specify validators by BlockId
    BlockId(self::block_id::BlockId),
}
