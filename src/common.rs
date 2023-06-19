use color_eyre::eyre::{Context, ContextCompat};
use num_rational::Rational32;
use prettytable::Table;

use near_jsonrpc_client::methods::{
    validators::RpcValidatorRequest,
    EXPERIMENTAL_genesis_config::{
        RpcGenesisConfigError, RpcGenesisConfigRequest, RpcGenesisConfigResponse,
    },
    EXPERIMENTAL_protocol_config::{
        RpcProtocolConfigError, RpcProtocolConfigRequest, RpcProtocolConfigResponse,
    },
};
use near_primitives::types::{BlockReference, EpochReference, Finality};
use near_primitives::views::{
    CurrentEpochValidatorInfo, EpochValidatorInfo, NextEpochValidatorInfo,
};

use near_cli_rs::common::JsonRpcClientExt;

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
        find_seat_price(
            current_validators
                .iter()
                .cloned()
                .map(CurrentOrNextValidatorInfoOrProposalsTable::from)
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

pub fn display_proposals_info(
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let json_rpc_client = network_config.json_rpc_client();
    let epoch_validator_info = json_rpc_client
        .blocking_call(&RpcValidatorRequest {
            epoch_reference: EpochReference::Latest,
        })
        .wrap_err("Failed to get epoch validators information request.")?;
    let current_proposals = epoch_validator_info.current_proposals;
    let current_proposals_stake: std::collections::HashMap<
        near_primitives::types::AccountId,
        near_primitives::types::Balance,
    > = current_proposals
        .clone()
        .into_iter()
        .map(|validator_stake_view| {
            let validator_stake = validator_stake_view.into_validator_stake();
            validator_stake.account_and_stake()
        })
        .collect();
    let current_validators = epoch_validator_info.current_validators;
    let current_validators_stake: std::collections::HashMap<
        near_primitives::types::AccountId,
        near_primitives::types::Balance,
    > = current_validators
        .into_iter()
        .map(|current_epoch_validator_info| {
            (
                current_epoch_validator_info.account_id,
                current_epoch_validator_info.stake,
            )
        })
        .collect();
    let mut combine_validators_and_proposals: std::collections::HashMap<
        near_primitives::types::AccountId,
        ProposalsTable,
    > = std::collections::HashMap::new();
    for (account_id, stake) in current_validators_stake {
        if let Some(new_stake) = current_proposals_stake.get(&account_id) {
            let proposals_table = ProposalsTable {
                account_id: account_id.clone(),
                status: "Proposal(Accepted)".to_string(),
                stake,
                new_stake: Some(new_stake.clone()),
            };
            combine_validators_and_proposals.insert(account_id, proposals_table)
        } else {
            let proposals_table = ProposalsTable {
                account_id: account_id.clone(),
                status: "Rollover".to_string(),
                stake,
                new_stake: None,
            };
            combine_validators_and_proposals.insert(account_id, proposals_table)
        };
    }
    let mut combine_validators_and_proposals_table: Vec<ProposalsTable> =
        combine_validators_and_proposals.into_values().collect();
    combine_validators_and_proposals_table.sort_by(|a, b| b.stake.cmp(&a.stake));
    // for row in &table {
    // let new_stake = match row.new_stake {
    //     Some(new_stake) => near_cli_rs::common::NearBalance::from_yoctonear(new_stake).to_string(),
    //     None => "".to_string()
    // };
    //     println!(
    //         "{:<20}  {:<42}  {:>65}  {:>65}",
    //         row.status,
    //         row.account_id,
    //         near_cli_rs::common::NearBalance::from_yoctonear(row.stake),
    //         new_stake
    //     )
    // }

    let genesis_config = json_rpc_client
        .blocking_call(&RpcGenesisConfigRequest)
        .wrap_err("Failed to get genesis config.")?;

    let protocol_config = json_rpc_client
        .blocking_call(&RpcProtocolConfigRequest {
            block_reference: BlockReference::Finality(Finality::Final),
        })
        .wrap_err("Failed to get protocol config.")?;

    let max_number_of_seats = protocol_config.num_block_producer_seats
        + protocol_config
            .avg_hidden_validator_seats_per_shard
            .iter()
            .sum::<u64>();
    let seat_price = find_seat_price(
        combine_validators_and_proposals_table
            .iter()
            .cloned()
            .map(CurrentOrNextValidatorInfoOrProposalsTable::from)
            .collect(),
        max_number_of_seats,
        genesis_config.minimum_stake_ratio,
        protocol_config.protocol_version,
    )?;
    println!("seat_price: {seat_price}");

    // current_proposals.sort_by(|a, b| b.clone().into_validator_stake().stake().cmp(&a.clone().into_validator_stake().stake()));
    // println!("current_proposals: {:#?}", current_proposals);
    println!("Proposals for the epoch after next (new: {}, passing: 216 XXX, expected seat price = 25,941 XXX)",
        current_proposals.len()
);

    let mut table = Table::new();
    table.set_titles(prettytable::row![Fg=>"Status", "Validator Id", "Stake", "New Stake"]);

    for validator_stake in combine_validators_and_proposals_table {
        let new_stake = match validator_stake.new_stake {
            Some(new_stake) => {
                near_cli_rs::common::NearBalance::from_yoctonear(new_stake).to_string()
            }
            None => "".to_string(),
        };

        table.add_row(prettytable::row![
            validator_stake.status,
            validator_stake.account_id,
            near_cli_rs::common::NearBalance::from_yoctonear(validator_stake.stake),
            new_stake
        ]);
        // println!("{:<42}  {:>38}", validator_stake.account_id().clone(), validator_stake.into_validator_stake().stake())
        // let approval_stake = validator_stake
        //     .into_validator_stake()
        //     .get_approval_stake(false);
        // println!(
        //     "{:<42}  {:>38}  {:>38}",
        //     approval_stake.account_id,
        //     approval_stake.stake_this_epoch,
        //     approval_stake.stake_next_epoch
        // )
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}

fn find_seat_price(
    validators: Vec<CurrentOrNextValidatorInfoOrProposalsTable>,
    max_number_of_seats: u64,
    minimum_stake_ratio: Rational32,
    protocol_version: near_primitives::types::ProtocolVersion,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    if protocol_version < 49 {
        return find_seat_price_for_protocol_before_49(validators, max_number_of_seats);
    }
    return find_seat_price_for_protocol_after_49(
        validators,
        max_number_of_seats,
        vec![
            minimum_stake_ratio.numer().clone(),
            minimum_stake_ratio.denom().clone(),
        ],
    );
}

fn find_seat_price_for_protocol_before_49(
    validators: Vec<CurrentOrNextValidatorInfoOrProposalsTable>,
    num_seats: u64,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    let stakes = validators
        .iter()
        .map(|validator_info| match validator_info {
            CurrentOrNextValidatorInfoOrProposalsTable::CurrentEpochValidatorInfo(current) => {
                current.stake
            }
            CurrentOrNextValidatorInfoOrProposalsTable::NextEpochValidatorInfo(next) => next.stake,
            CurrentOrNextValidatorInfoOrProposalsTable::ProposalsTable(proposals_table) => {
                proposals_table.stake
            }
        })
        .collect::<Vec<_>>();
    let stakes_sum: u128 = stakes.iter().sum();
    if stakes_sum < num_seats as u128 {
        return Err(color_eyre::eyre::Report::msg("Stakes are below seats"));
    }
    let mut left: u128 = 1;
    let mut right: u128 = stakes_sum + 1;
    while left != (right - 1) {
        let mid = left.saturating_add(right) / 2;
        let mut found = false;
        let mut current_sum: u128 = 0;
        for stake in stakes.clone() {
            current_sum = current_sum.saturating_add(stake.saturating_div(mid));
            if current_sum >= num_seats as u128 {
                left = mid;
                found = true;
                break;
            }
        }
        if !found {
            right = mid;
        }
    }
    Ok(near_cli_rs::common::NearBalance::from_yoctonear(left))
}

fn find_seat_price_for_protocol_after_49(
    validators: Vec<CurrentOrNextValidatorInfoOrProposalsTable>,
    max_number_of_seats: u64,
    minimum_stake_ratio: Vec<i32>,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    if minimum_stake_ratio.len() != 2 {
        return Err(color_eyre::eyre::Report::msg(
            "Error: minimumStakeRatio should have 2 elements",
        ));
    }
    let mut stakes = validators
        .iter()
        .map(|validator_info| match validator_info {
            CurrentOrNextValidatorInfoOrProposalsTable::CurrentEpochValidatorInfo(current) => {
                current.stake
            }
            CurrentOrNextValidatorInfoOrProposalsTable::NextEpochValidatorInfo(next) => next.stake,
            CurrentOrNextValidatorInfoOrProposalsTable::ProposalsTable(proposals_table) => {
                proposals_table.stake
            }
        })
        .collect::<Vec<_>>();

    let stakes_sum: u128 = stakes.iter().sum();
    if validators.len() < max_number_of_seats as usize {
        return Ok(near_cli_rs::common::NearBalance::from_yoctonear(
            stakes_sum
                .checked_mul(minimum_stake_ratio[0] as u128)
                .wrap_err("Can't multiply these numbers")?
                .checked_div(minimum_stake_ratio[1] as u128)
                .wrap_err("Can't divide these numbers")?,
        ));
    };

    stakes.sort();

    Ok(near_cli_rs::common::NearBalance::from_yoctonear(
        stakes[0] + 1,
    ))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum CurrentOrNextValidatorInfoOrProposalsTable {
    #[serde(rename = "current_epoch_validator_info")]
    CurrentEpochValidatorInfo(CurrentEpochValidatorInfo),
    #[serde(rename = "next_epoch_validator_info")]
    NextEpochValidatorInfo(NextEpochValidatorInfo),
    #[serde(rename = "proposals_table")]
    ProposalsTable(ProposalsTable),
}

impl From<CurrentEpochValidatorInfo> for CurrentOrNextValidatorInfoOrProposalsTable {
    fn from(current_epoch_validator_info: CurrentEpochValidatorInfo) -> Self {
        Self::CurrentEpochValidatorInfo(current_epoch_validator_info)
    }
}

impl From<NextEpochValidatorInfo> for CurrentOrNextValidatorInfoOrProposalsTable {
    fn from(next_epoch_validator_info: NextEpochValidatorInfo) -> Self {
        Self::NextEpochValidatorInfo(next_epoch_validator_info)
    }
}

impl From<ProposalsTable> for CurrentOrNextValidatorInfoOrProposalsTable {
    fn from(proposals_table: ProposalsTable) -> Self {
        Self::ProposalsTable(proposals_table)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
struct ProposalsTable {
    account_id: near_primitives::types::AccountId,
    status: String,
    stake: near_primitives::types::Balance,
    new_stake: Option<near_primitives::types::Balance>,
}
