#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = EpochIdContext)]
pub struct EpochId {
    /// Type the ipoch ID hash:
    epoch_id: near_cli_rs::types::crypto_hash::CryptoHash,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: near_cli_rs::network::Network,
}

#[derive(Clone)]
pub struct EpochIdContext(near_cli_rs::network::NetworkContext);

impl EpochIdContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        scope: &<EpochId as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let epoch_id: near_primitives::hash::CryptoHash = scope.epoch_id.into();
        let on_after_getting_network_callback: near_cli_rs::network::OnAfterGettingNetworkCallback =
            std::sync::Arc::new({
                move |network_config| {
                    crate::common::display_validators_info(
                        near_primitives::types::EpochReference::EpochId(
                            near_primitives::types::EpochId(epoch_id),
                        ),
                        network_config,
                    )
                }
            });
        Ok(Self(near_cli_rs::network::NetworkContext {
            config: previous_context.config,
            on_after_getting_network_callback,
        }))
    }
}

impl From<EpochIdContext> for near_cli_rs::network::NetworkContext {
    fn from(item: EpochIdContext) -> Self {
        item.0
    }
}
