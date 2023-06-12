use near_primitives::types::BlockId;
use std::str::FromStr;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub type OnAfterGettingBlockReferenceCallback =
    std::sync::Arc<dyn Fn(&near_cli_rs::config::NetworkConfig, &BlockId) -> crate::CliResult>;

#[derive(Clone)]
pub struct ArgsForViewContext {
    pub config: near_cli_rs::config::Config,
    pub on_after_getting_block_reference_callback: OnAfterGettingBlockReferenceCallback,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ArgsForViewContext)]
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
    network_config: near_cli_rs::config::NetworkConfig,
    on_after_getting_block_reference_callback: OnAfterGettingBlockReferenceCallback,
}

impl NetworkViewAtBlockArgsContext {
    pub fn from_previous_context(
        previous_context: ArgsForViewContext,
        scope: &<NetworkViewAtBlockArgs as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let network_connection = previous_context.config.network_connection.clone();
        let network_config = network_connection
            .get(&scope.network_name)
            .expect("Failed to get network config!")
            .clone();
        Ok(Self {
            network_config,
            on_after_getting_block_reference_callback: previous_context
                .on_after_getting_block_reference_callback,
        })
    }
}

impl NetworkViewAtBlockArgs {
    fn input_network_name(
        context: &ArgsForViewContext,
    ) -> color_eyre::eyre::Result<Option<String>> {
        near_cli_rs::common::input_network_name(&context.config)
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = NetworkViewAtBlockArgsContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Сhoose block for view:
pub enum ViewAtBlock {
    #[strum_discriminants(strum(
        message = "at-block-height   - View properties in a height-selected block"
    ))]
    /// View properties in a height-selected block
    AtBlockHeight(AtBlockHeight),
    #[strum_discriminants(strum(
        message = "at-block-hash     - View properties in a hash-selected block"
    ))]
    /// View properties in a hash-selected block
    AtBlockHash(BlockIdHash),
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = AtBlockHeightContext)]
pub struct AtBlockHeight {
    /// Type the block ID height:
    block_id_height: near_primitives::types::BlockHeight,
}

#[derive(Debug, Clone)]
pub struct AtBlockHeightContext;

impl AtBlockHeightContext {
    pub fn from_previous_context(
        previous_context: NetworkViewAtBlockArgsContext,
        scope: &<AtBlockHeight as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let block_id = BlockId::Height(scope.block_id_height);

        (previous_context.on_after_getting_block_reference_callback)(
            &previous_context.network_config,
            &block_id,
        )?;
        Ok(Self)
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = BlockIdHashContext)]
pub struct BlockIdHash {
    /// Type the block ID hash:
    block_id_hash: near_cli_rs::types::crypto_hash::CryptoHash,
}

#[derive(Debug, Clone)]
pub struct BlockIdHashContext;

impl BlockIdHashContext {
    pub fn from_previous_context(
        previous_context: NetworkViewAtBlockArgsContext,
        scope: &<BlockIdHash as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let block_id = BlockId::Hash(
            near_primitives::hash::CryptoHash::from_str(&scope.block_id_hash.to_string()).unwrap(),
        );

        (previous_context.on_after_getting_block_reference_callback)(
            &previous_context.network_config,
            &block_id,
        )?;
        Ok(Self)
    }
}
