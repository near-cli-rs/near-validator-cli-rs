use color_eyre::eyre::Context;

use near_cli_rs::common::{JsonRpcClientExt, RpcQueryResponseExt};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = ViewStakeContext)]
pub struct ViewStake {
    #[interactive_clap(skip_default_input_arg)]
    /// Enter validator account ID to view stake:
    validator_account_id: near_cli_rs::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: near_cli_rs::network_view_at_block::NetworkViewAtBlockArgs,
}

#[derive(Clone)]
pub struct ViewStakeContext(near_cli_rs::network_view_at_block::ArgsForViewContext);

impl ViewStakeContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        scope: &<ViewStake as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let on_after_getting_block_reference_callback: near_cli_rs::network_view_at_block::OnAfterGettingBlockReferenceCallback = std::sync::Arc::new({
            let validator_account_id: near_primitives::types::AccountId = scope.validator_account_id.clone().into();

            move |network_config, block_reference| {
                let json_rpc_client = network_config.json_rpc_client();

                let rpc_query_response = json_rpc_client
                    .blocking_call_view_account(&validator_account_id.clone(), block_reference.clone())
                    .wrap_err_with(|| {
                        format!(
                            "Failed to fetch query ViewAccount for <{}>",
                            &validator_account_id
                        )
                    })?;
                let account_view = rpc_query_response.account_view()?;
                eprintln!("Validator {validator_account_id} staked amount {}",
                    account_view.locked.exact_amount_display()
                );

                Ok(())
            }
        });
        Ok(Self(
            near_cli_rs::network_view_at_block::ArgsForViewContext {
                config: previous_context.config,
                interacting_with_account_ids: vec![scope.validator_account_id.clone().into()],
                on_after_getting_block_reference_callback,
            },
        ))
    }
}

impl From<ViewStakeContext> for near_cli_rs::network_view_at_block::ArgsForViewContext {
    fn from(item: ViewStakeContext) -> Self {
        item.0
    }
}

impl ViewStake {
    pub fn input_validator_account_id(
        context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<near_cli_rs::types::account_id::AccountId>> {
        near_cli_rs::common::input_non_signer_account_id_from_used_account_list(
            &context.config.credentials_home_dir,
            "What Account ID do you need to view?",
        )
    }
}
