use color_eyre::eyre::ContextCompat;
use num_rational::Rational32;

pub fn find_seat_price(
    stakes: Vec<u128>,
    max_number_of_seats: u64,
    minimum_stake_ratio: Rational32,
    protocol_version: near_primitives::types::ProtocolVersion,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    if protocol_version < 49 {
        return find_seat_price_for_protocol_before_49(stakes, max_number_of_seats);
    }
    find_seat_price_for_protocol_after_49(stakes, max_number_of_seats, minimum_stake_ratio)
}

/// This implementation is ported from near-api-js:
/// https://github.com/near/near-api-js/blob/bdbf839f54fbc399d7299da8cf9966bbdc426238/packages/utils/src/validators.ts#L24-L50
fn find_seat_price_for_protocol_before_49(
    stakes: Vec<u128>,
    num_seats: u64,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
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

/// This implementation is ported from near-api-js:
/// https://github.com/near/near-api-js/blob/bdbf839f54fbc399d7299da8cf9966bbdc426238/packages/utils/src/validators.ts#L52-L64
fn find_seat_price_for_protocol_after_49(
    mut stakes: Vec<u128>,
    max_number_of_seats: u64,
    minimum_stake_ratio: Rational32,
) -> color_eyre::eyre::Result<near_cli_rs::common::NearBalance> {
    let stakes_sum: u128 = stakes.iter().sum();
    if stakes.len() < max_number_of_seats as usize {
        return Ok(near_cli_rs::common::NearBalance::from_yoctonear(
            stakes_sum
                .checked_mul(*minimum_stake_ratio.numer() as u128)
                .wrap_err("Can't multiply these numbers")?
                .checked_div(*minimum_stake_ratio.denom() as u128)
                .wrap_err("Can't divide these numbers")?,
        ));
    };
    stakes.sort();

    Ok(near_cli_rs::common::NearBalance::from_yoctonear(
        stakes[0] + 1,
    ))
}
