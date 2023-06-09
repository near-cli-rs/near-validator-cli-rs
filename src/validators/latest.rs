#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = LatestContext)]
pub struct Latest {
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: near_cli_rs::network::Network,
}

#[derive(Clone)]
pub struct LatestContext(near_cli_rs::network::NetworkContext);

impl LatestContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        _scope: &<Latest as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let on_after_getting_network_callback: near_cli_rs::network::OnAfterGettingNetworkCallback =
            std::sync::Arc::new({
                move |network_config| {
                    super::display_validators_info(
                        near_primitives::types::EpochReference::Latest,
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

impl From<LatestContext> for near_cli_rs::network::NetworkContext {
    fn from(item: LatestContext) -> Self {
        item.0
    }
}
