mod block_id;
mod epoch_id;
mod network_view_at_block;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
pub struct Validators {
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: self::network_view_at_block::NetworkViewAtBlockArgs,
}
