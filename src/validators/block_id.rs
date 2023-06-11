#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = BlockIdContext)]
pub struct BlockId {
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_view_at_block::NetworkViewAtBlockArgs,
}

#[derive(Clone)]
pub struct BlockIdContext(crate::network_view_at_block::ArgsForViewContext);

impl BlockIdContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        _scope: &<BlockId as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let on_after_getting_block_reference_callback: crate::network_view_at_block::OnAfterGettingBlockReferenceCallback = std::sync::Arc::new({
            move |network_config, block_id| {
                crate::common::display_validators_info(
                    near_primitives::types::EpochReference::BlockId(
                        block_id.clone()
                    ),
                    network_config,
                )
            }
        });
        Ok(Self(crate::network_view_at_block::ArgsForViewContext {
            config: previous_context.config,
            on_after_getting_block_reference_callback,
        }))
    }
}

impl From<BlockIdContext> for crate::network_view_at_block::ArgsForViewContext {
    fn from(item: BlockIdContext) -> Self {
        item.0
    }
}
