use near_primitives::types::{BlockId, EpochReference};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::network_view_at_block::NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = AtBlockHeightContext)]
pub struct AtBlockHeight {
    /// Type the block ID height:
    block_id_height: near_primitives::types::BlockHeight,
}

#[derive(Debug, Clone)]
pub struct AtBlockHeightContext;

impl AtBlockHeightContext {
    pub fn from_previous_context(
        previous_context: super::network_view_at_block::NetworkViewAtBlockArgsContext,
        scope: &<AtBlockHeight as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let epoch_reference = EpochReference::BlockId(BlockId::Height(scope.block_id_height));
        super::epoch_id::latest::display_current_validators_info(
            epoch_reference,
            &previous_context.network_config,
        )?;
        Ok(Self)
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::network_view_at_block::NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = BlockIdHashContext)]
pub struct BlockIdHash {
    /// Type the block ID hash:
    block_id_hash: near_cli_rs::types::crypto_hash::CryptoHash,
}

#[derive(Debug, Clone)]
pub struct BlockIdHashContext;

impl BlockIdHashContext {
    pub fn from_previous_context(
        previous_context: super::network_view_at_block::NetworkViewAtBlockArgsContext,
        scope: &<BlockIdHash as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let epoch_reference = EpochReference::BlockId(BlockId::Hash(scope.block_id_hash.into()));
        super::epoch_id::latest::display_current_validators_info(
            epoch_reference,
            &previous_context.network_config,
        )?;
        Ok(Self)
    }
}
