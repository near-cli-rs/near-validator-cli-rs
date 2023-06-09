use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use near_cli_rs::common::JsonRpcClientExt;
use near_jsonrpc_client::methods::EXPERIMENTAL_genesis_config::{
    RpcGenesisConfigRequest, RpcGenesisConfigResponse,
};
use near_primitives::{borsh::BorshDeserialize, views::EpochValidatorInfo};

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

fn validators_info(
    epoch: near_primitives::types::EpochReference,
    network_config: &near_cli_rs::config::NetworkConfig,
) -> (RpcGenesisConfigResponse, EpochValidatorInfo) {
    // let client = near_jsonrpc_client::JsonRpcClient::connect(network_config.rpc_url.as_str());
    let client = network_config.json_rpc_client();
    let genesis_config = client.blocking_call(RpcGenesisConfigRequest).unwrap();

    // let genesis_config_request = RpcGenesisConfigRequest;

    // let genesis_config = client.clone().call(&genesis_config_request).await.unwrap();

    let validators_request = near_jsonrpc_client::methods::validators::RpcValidatorRequest {
        epoch_reference: epoch,
    };

    let validator_info = client.blocking_call(&validators_request).unwrap();

    (genesis_config, validator_info)
}

pub fn display_validators_info(
    epoch: near_primitives::types::EpochReference,
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let (genesis_config, validator_info) = validators_info(epoch, network_config);

    //TODO: make it pretty
    println!("-------------- Validators info (should be in table) ----------------------");
    println!("Genesis config: {:?}", genesis_config);
    println!("------------------------------------");
    println!("Validator info: {:?}", validator_info);

    Ok(())
}

fn display_proposals_info(network_config: &near_cli_rs::config::NetworkConfig) -> crate::CliResult {
    let (genesis_config, validator_info) = validators_info(
        near_primitives::types::EpochReference::Latest,
        network_config,
    );

    //TODO: make it pretty
    println!("-------------- Proposals info (should be in table) ----------------------");
    println!("Genesis config: {:?}", genesis_config);
    println!("------------------------------------");
    println!("Validator info: {:?}", validator_info);

    Ok(())
}
