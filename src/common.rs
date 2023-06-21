use color_eyre::eyre::ContextCompat;
use num_rational::Rational32;

use near_primitives::views::{CurrentEpochValidatorInfo, NextEpochValidatorInfo};

pub fn find_seat_price(
    validators: Vec<CurrentOrNextValidatorInfoOrProposalsTable>,
    max_number_of_seats: u64,
    minimum_stake_ratio: Rational32,
    protocol_version: near_primitives::types::ProtocolVersion,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    if protocol_version < 49 {
        return find_seat_price_for_protocol_before_49(validators, max_number_of_seats);
    }
    find_seat_price_for_protocol_after_49(
        validators,
        max_number_of_seats,
        vec![*minimum_stake_ratio.numer(), *minimum_stake_ratio.denom()],
    )
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
    if stakes_sum < num_seats.into() {
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
                match proposals_table.new_stake {
                    Some(new_stake) => new_stake,
                    None => proposals_table.stake,
                }
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
pub enum CurrentOrNextValidatorInfoOrProposalsTable {
    #[serde(rename = "current_epoch_validator_info")]
    CurrentEpochValidatorInfo(CurrentEpochValidatorInfo),
    #[serde(rename = "next_epoch_validator_info")]
    NextEpochValidatorInfo(NextEpochValidatorInfo),
    #[serde(rename = "proposals_table")]
    ProposalsTable(crate::proposals::ProposalsTable),
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

impl From<crate::proposals::ProposalsTable> for CurrentOrNextValidatorInfoOrProposalsTable {
    fn from(proposals_table: crate::proposals::ProposalsTable) -> Self {
        Self::ProposalsTable(proposals_table)
    }
}
