use near_cli_rs::common::JsonRpcClientExt;
use near_jsonrpc_client::methods::{
    validators::RpcValidatorRequest,
    EXPERIMENTAL_genesis_config::{RpcGenesisConfigRequest, RpcGenesisConfigResponse},
};
use near_primitives::views::EpochValidatorInfo;

fn validators_info(
    epoch: near_primitives::types::EpochReference,
    network_config: &near_cli_rs::config::NetworkConfig,
) -> (RpcGenesisConfigResponse, EpochValidatorInfo) {
    let client = network_config.json_rpc_client();
    let genesis_config = client.blocking_call(RpcGenesisConfigRequest).unwrap();
    let validator_info = client
        .blocking_call(&RpcValidatorRequest {
            epoch_reference: epoch,
        })
        .unwrap();
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

pub fn display_proposals_info(
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
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
