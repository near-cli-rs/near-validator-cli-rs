use color_eyre::eyre::Context;
use prettytable::Table;

use near_jsonrpc_client::methods::{
    validators::RpcValidatorRequest, EXPERIMENTAL_genesis_config::RpcGenesisConfigRequest,
    EXPERIMENTAL_protocol_config::RpcProtocolConfigRequest,
};
use near_primitives::types::{BlockReference, EpochReference, Finality};

use near_cli_rs::common::JsonRpcClientExt;

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

pub fn display_validators_info(
    epoch_reference: near_primitives::types::EpochReference,
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let block_reference = match &epoch_reference {
        EpochReference::Latest => BlockReference::Finality(Finality::Final),
        EpochReference::BlockId(block_id) => BlockReference::BlockId(block_id.clone()),
        _ => {
            return Err(color_eyre::eyre::ErrReport::msg(
                "BlockReference: incorrect value entered",
            ))
        }
    };

    let json_rpc_client = network_config.json_rpc_client();

    let mut current_validators = json_rpc_client
        .blocking_call(&RpcValidatorRequest { epoch_reference })
        .wrap_err("Failed to get epoch validators information request.")?
        .current_validators;
    current_validators.sort_by(|a, b| b.stake.cmp(&a.stake));

    let genesis_config = json_rpc_client
        .blocking_call(&RpcGenesisConfigRequest)
        .wrap_err("Failed to get genesis config.")?;

    let protocol_config = json_rpc_client
        .blocking_call(&RpcProtocolConfigRequest { block_reference })
        .wrap_err("Failed to get protocol config.")?;

    let max_number_of_seats = protocol_config.num_block_producer_seats
        + protocol_config
            .avg_hidden_validator_seats_per_shard
            .iter()
            .sum::<u64>();
    println!(
        "Validators (total: {}, seat price: {}",
        current_validators.len(),
        crate::common::find_seat_price(
            current_validators
                .iter()
                .cloned()
                .map(crate::common::CurrentOrNextValidatorInfoOrProposalsTable::from)
                .collect(),
            max_number_of_seats,
            genesis_config.minimum_stake_ratio,
            protocol_config.protocol_version
        )?
    );

    let mut table = Table::new();
    table.set_titles(prettytable::row![Fg=>"Validator Id", "Stake", "Online", "Blocks produced", "Blocks expected", "Chunks produced", "Chunks expected"]);

    for validator in &current_validators {
        let online = if validator.num_expected_blocks + validator.num_expected_chunks == 0 {
            "NaN".to_string()
        } else {
            format!(
                "{:>6.2} %",
                ((validator.num_produced_blocks + validator.num_produced_chunks) * 100) as f64
                    / (validator.num_expected_blocks + validator.num_expected_chunks) as f64
            )
        };
        table.add_row(prettytable::row![
            validator.account_id,
            near_cli_rs::common::NearBalance::from_yoctonear(validator.stake),
            online,
            validator.num_produced_blocks,
            validator.num_expected_blocks,
            validator.num_produced_chunks,
            validator.num_expected_chunks
        ]);
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}
