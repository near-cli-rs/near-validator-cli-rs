use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = NetworkViewAtBlockArgsContext)]
pub struct NetworkViewAtBlockArgs {
    /// What is the name of the network?
    #[interactive_clap(skip_default_input_arg)]
    network_name: String,
    #[interactive_clap(subcommand)]
    next: ViewAtBlock,
}

#[derive(Clone)]
pub struct NetworkViewAtBlockArgsContext {
    pub network_config: near_cli_rs::config::NetworkConfig,
}

impl NetworkViewAtBlockArgsContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        scope: &<NetworkViewAtBlockArgs as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let network_connection = previous_context.config.network_connection;
        let network_config = network_connection
            .get(&scope.network_name)
            .expect("Failed to get network config!")
            .clone();
        Ok(Self { network_config })
    }
}

impl NetworkViewAtBlockArgs {
    fn input_network_name(
        context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<String>> {
        near_cli_rs::common::input_network_name(&context.config)
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = NetworkViewAtBlockArgsContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Choose block for view:
pub enum ViewAtBlock {
    #[strum_discriminants(strum(message = "latest            -   View latest validators"))]
    /// View latest validators
    Now(super::epoch_id::latest::Latest),
    #[strum_discriminants(strum(message = "next              -   View next validators"))]
    /// View next validators
    Next(super::epoch_id::next::Next),
    #[strum_discriminants(strum(
        message = "at-block-height   - View validators in a height-selected block"
    ))]
    /// View validators in a height-selected block
    AtBlockHeight(super::block_id::AtBlockHeight),
    #[strum_discriminants(strum(
        message = "at-block-hash     - View validators in a hash-selected block"
    ))]
    /// View validators in a hash-selected block
    AtBlockHash(super::block_id::BlockIdHash),
}
