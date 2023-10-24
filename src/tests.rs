use super::{ pallet::pallet::Error, pallet::pallet::Event, * };
use mock::*;
use frame_support::{ assert_noop, assert_ok, traits::fungibles };
use sp_runtime::{ FixedU128, FixedPointNumber };
use testing_utils::*;
use crate::Config;

#[test]
fn initialize() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();

		assert_eq!(Bonds::current_era(), 1);
	})
}

#[test]
fn new_era_length_is_always_blocks_per_era() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		let blocks_per_era = mock::BLOCKS_PER_ERA;

		// go to beginning of an era
		advance_to_era(mock::Bonds::current_era() + 1);

		// record era number and block number
		let start_era = mock::Bonds::current_era();
		let starting_block_number = System::block_number();

		// go to next era
		advance_to_era(mock::Bonds::current_era() + 1);
		let ending_block_number = System::block_number();

		// make sure block number difference is is blocks_per_era
		assert_eq!(mock::Bonds::current_era(), start_era + 1);
		assert_eq!(ending_block_number - starting_block_number, blocks_per_era);
	})
}

#[test]
fn general_staker_info_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let (staker_1, staker_2, staker_3) = (1, 2, 3);
		let amount = 100;

		let starting_era = 3;
		advance_to_era(starting_era);
		assert_bond_stake(staker_1, amount, ASSET_ID);
		assert_bond_stake(staker_2, amount, ASSET_ID);

		let mid_era = 7;
		advance_to_era(mid_era);
		assert_bond_stake(staker_3, amount, ASSET_ID);

		let final_era = 12;
		advance_to_era(final_era);

		// Check first interval
		let mut first_staker_info = Bonds::asset_staker_info(ASSET_ID, &staker_1);
		let mut second_staker_info = Bonds::asset_staker_info(ASSET_ID, &staker_2);
		let mut third_staker_info = Bonds::asset_staker_info(ASSET_ID, &staker_3);

		for era in starting_era..mid_era {
			assert_eq!((era, amount), first_staker_info.claim(era, 0u64));
			assert_eq!((era, amount), second_staker_info.claim(era, 0u64));
		}

		// Check second interval
		for era in mid_era..=final_era {
			assert_eq!((era, amount), first_staker_info.claim(era, 0u64));
			assert_eq!((era, amount), third_staker_info.claim(era, 0u64));
		}
	})
}

#[test]
fn bond_stake_different_eras_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;

		// initially, storage values should be None
		let current_era = Bonds::current_era();

		assert_bond_stake(staker_id, 100, ASSET_ID);

		advance_to_era(current_era + 2);

		// Stake and bond again but using a different amount.
		assert_bond_stake(staker_id, 300, ASSET_ID);
	})
}

#[test]
fn bond_stake_different_value_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;

		// Bond&stake almost the entire available balance of the staker.
		let staker_free_balance = Assets::balance(ASSET_ID, &staker_id).saturating_sub(
			MINIMUM_REMAINING_AMOUNT
		);
		assert_bond_stake(staker_id, staker_free_balance - 1, ASSET_ID);

		// Bond&stake again.
		assert_bond_stake(staker_id, 1, ASSET_ID);
	})
}

#[test]
fn unbond_unstake_multiple_time_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;
		let original_staked_value = 300 + MINIMUM_STAKING_AMOUNT;
		let old_era = Bonds::current_era();

		//bond&stake.
		assert_bond_stake(staker_id, original_staked_value, ASSET_ID);
		advance_to_era(old_era + 1);

		// Unstake such an amount so there will remain staked funds
		let unstaked_value = 100;
		assert_unbond_unstake(staker_id, unstaked_value, ASSET_ID);

		// Unbond yet again, but don't advance era
		// Unstake such an amount so there will remain staked funds
		let unstaked_value = 50;
		assert_unbond_unstake(staker_id, unstaked_value, ASSET_ID);
	})
}

#[test]
fn unbond_unstake_value_below_staking_threshold() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let era = 1;
		let staker_id = 1;
		let first_value_to_unstake = 300;
		let staked_value = first_value_to_unstake + MINIMUM_STAKING_AMOUNT;

		assert_bond_stake(staker_id, staked_value, ASSET_ID);

		// Unstake such an amount that exactly minimum staking amount will remain staked.
		assert_unbond_unstake(staker_id, first_value_to_unstake, ASSET_ID);

		// Unstake 1 token and expect that the entire staked amount will be unstaked.
		assert_unbond_unstake(staker_id, 1, ASSET_ID);

		assert_eq!(Bonds::asset_general_era_info(ASSET_ID, era).unwrap().staked, 0);

		assert_eq!(Bonds::asset_general_era_info(ASSET_ID, era).unwrap().locked, staked_value);
	})
}

#[test]
fn unbond_unstake_in_different_eras() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let (first_staker_id, second_staker_id) = (1, 2);
		let staked_value = 500;

		assert_bond_stake(first_staker_id, staked_value, ASSET_ID);
		assert_bond_stake(second_staker_id, staked_value, ASSET_ID);

		// Advance era, unbond&withdraw with first staker, verify that it was successful
		advance_to_era(Bonds::current_era() + 10);
		let current_era = Bonds::current_era();
		assert_unbond_unstake(first_staker_id, 100, ASSET_ID);

		// Advance era, unbond with second staker and verify storage values are as expected
		advance_to_era(current_era + 10);
		assert_unbond_unstake(second_staker_id, 333, ASSET_ID);
	})
}

#[test]
fn unbond_unstake_calls_in_same_era_can_exceed_max_chunks() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker = 1;
		assert_bond_stake(staker, 200 * (MAX_UNLOCKING_CHUNKS as Balance), ASSET_ID);

		// Ensure that we can unbond up to a limited amount of time.
		for _ in 0..MAX_UNLOCKING_CHUNKS * 2 {
			assert_unbond_unstake(1, 10, ASSET_ID);
			assert_eq!(1, AssetLedger::<Test>::get(ASSET_ID, &staker).unbonding_info.len());
		}
	})
}

#[test]
fn unbond_unstake_with_zero_value_is_not_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		assert_noop!(
			Bonds::asset_unbond_and_unstake(RuntimeOrigin::signed(1), ASSET_ID, 0),
			Error::<Test>::UnstakingWithNoValue
		);
	})
}

#[test]
fn unbond_unstake_too_many_unlocking_chunks_is_not_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker = 1;
		let unstake_amount = 10;
		let stake_amount =
			MINIMUM_STAKING_AMOUNT * 10 + unstake_amount * (MAX_UNLOCKING_CHUNKS as Balance);

		assert_bond_stake(staker, stake_amount, ASSET_ID);

		// Ensure that we can unbond up to a limited amount of time.
		for _ in 0..MAX_UNLOCKING_CHUNKS {
			advance_to_era(Bonds::current_era() + 1);
			assert_unbond_unstake(staker, unstake_amount, ASSET_ID);
		}

		// Ensure that we're at the max but can still add new chunks since it should be merged with the existing one
		assert_eq!(
			MAX_UNLOCKING_CHUNKS,
			Bonds::asset_ledger(ASSET_ID, &staker).unbonding_info.len()
		);
		assert_unbond_unstake(staker, unstake_amount, ASSET_ID);

		// Ensure that further unbonding attempts result in an error.
		advance_to_era(Bonds::current_era() + 1);
		assert_noop!(
			Bonds::asset_unbond_and_unstake(
				RuntimeOrigin::signed(staker),
				ASSET_ID,
				unstake_amount
			),
			Error::<Test>::TooManyUnlockingChunks
		);
	})
}

#[test]
fn unbond_unstake_too_many_era_stakes() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);
		let staker_id = 1;

		// Fill up the `EraStakes` vec
		let start_era = Bonds::current_era();
		for offset in 1..MAX_ERA_STAKE_VALUES {
			assert_bond_stake(staker_id, 100, ASSET_ID);
			advance_to_era(start_era + offset);
		}

		// At this point, we have max allowed amount of `EraStake` values so we cannot create
		// an additional one.
		assert_noop!(
			Bonds::asset_unbond_and_unstake(RuntimeOrigin::signed(staker_id), ASSET_ID, 10),
			Error::<Test>::TooManyEraStakeValues
		);
	})
}

#[test]
fn withdraw_unbonded_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;
		assert_bond_stake(staker_id, 1000, ASSET_ID);

		let first_unbond_value = 75;
		let second_unbond_value = 39;
		let initial_era = Bonds::current_era();

		// Unbond some amount in the initial era
		assert_unbond_unstake(staker_id, first_unbond_value, ASSET_ID);

		// Advance one era and then unbond some more
		advance_to_era(initial_era + 1);
		assert_unbond_unstake(staker_id, second_unbond_value, ASSET_ID);

		// Now advance one era before first chunks finishes the unbonding process
		advance_to_era(initial_era + UNBONDING_PERIOD - 1);
		assert_noop!(
			Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(staker_id), ASSET_ID),
			Error::<Test>::NothingToWithdraw
		);

		// Advance one additional era and expect that the first chunk can be withdrawn
		advance_to_era(Bonds::current_era() + 1);
		assert_ok!(Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(staker_id), ASSET_ID));
		System::assert_last_event(
			RuntimeEvent::Bonds(Event::WithdrawnFromAsset(staker_id, ASSET_ID, first_unbond_value))
		);

		// Advance one additional era and expect that the first chunk can be withdrawn
		advance_to_era(Bonds::current_era() + 1);
		assert_ok!(Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(staker_id), ASSET_ID));
		System::assert_last_event(
			RuntimeEvent::Bonds(Event::WithdrawnFromAsset(staker_id, ASSET_ID, second_unbond_value))
		);

		// Advance one additional era but since we have nothing else to withdraw, expect an error
		advance_to_era(initial_era + UNBONDING_PERIOD - 1);
		assert_noop!(
			Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(staker_id), ASSET_ID),
			Error::<Test>::NothingToWithdraw
		);
	})
}

#[test]
fn withdraw_unbonded_full_vector_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;
		assert_bond_stake(staker_id, 1000, ASSET_ID);

		// Repeatedly start unbonding and advance era to create unlocking chunks
		let init_unbonding_amount = 1;
		for x in 1..=MAX_UNLOCKING_CHUNKS {
			assert_unbond_unstake(staker_id, init_unbonding_amount * (x as u128), ASSET_ID);
			advance_to_era(Bonds::current_era() + 1);
		}

		// Now clean up all that are eligible for clean-up
		assert_withdraw_unbonded(staker_id);

		// This is a sanity check for the test. Some chunks should remain, otherwise test isn't testing realistic unbonding period.
		assert!(!AssetLedger::<Test>::get(ASSET_ID, &staker_id).unbonding_info.is_empty());

		while !AssetLedger::<Test>::get(ASSET_ID, &staker_id).unbonding_info.is_empty() {
			advance_to_era(Bonds::current_era() + 1);
			assert_withdraw_unbonded(staker_id);
		}
	})
}

#[test]
fn withdraw_unbonded_no_value_is_not_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();

		assert_noop!(
			Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(1), ASSET_ID),
			Error::<Test>::NothingToWithdraw
		);
	})
}

#[test]
fn claim_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let first_staker = 1;
		let second_staker = 2;

		let start_era = Bonds::current_era();

		// Prepare a scenario with different stakes
		assert_bond_stake(first_staker, 10000, ASSET_ID);
		assert_bond_stake(second_staker, 45, ASSET_ID);

		let eras_advanced = 3;
		advance_to_era(start_era + eras_advanced);

		// Ensure that all past eras can be claimed
		let current_era = Bonds::current_era();
		for _ in start_era..current_era {
			assert_claim_staking_reward(first_staker);
			assert_claim_staking_reward(second_staker);
		}

		// Shouldn't be possible to claim current era.
		// Also, previous claim calls should have claimed everything prior to current era.
		assert_noop!(
			Bonds::claim_asset_rewards(RuntimeOrigin::signed(first_staker), ASSET_ID),
			Error::<Test>::EraOutOfBounds
		);
	})
}

#[test]
fn staking_for_a_year() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		<Test as Config>::Currency::make_free_balance_be(&Bonds::account_id(), 1_000_000);

		let staker_id = 1;
		let staked_value = 100000;
		assert_bond_stake(staker_id, staked_value, ASSET_ID);

		let start_era = Bonds::current_era();

		let final_era = 365;

		// Claim eras
		for era in 1..final_era {
			advance_to_era(start_era + era);
			assert_claim_staking_reward(staker_id);
		}

		let staker_info_after_1_year = Bonds::asset_staker_info(ASSET_ID, staker_id);
		let apr = Bonds::asset_apr(ASSET_ID);

		assert!(
			FixedU128::from_rational(apr.into(), 36500u128).saturating_mul_int(staked_value) <
				staker_info_after_1_year.latest_staked_value()
		);
	})
}

#[test]
fn claim_after_7_days() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		<Test as Config>::Currency::make_free_balance_be(&Bonds::account_id(), 1_000_000);

		let staker_id = 1;
		let staked_value = 100000;
		assert_bond_stake(staker_id, staked_value, ASSET_ID);

		let final_era = 10;

		// Move eras without claiming
		advance_to_era(final_era);

		// Claim should clean rewards outdated
		assert_claim_staking_reward(staker_id);

		let staker_info_after_claim = Bonds::asset_staker_info(ASSET_ID, staker_id);

		// Claim should remove the era 1,2 and claim 3
		// Only the reward for one era should be claimed and the next claimable era should be 4
		assert_eq!(staker_info_after_claim.stakes[0].era, 4);
		assert_eq!(
			staker_info_after_claim.latest_staked_value(),
			staked_value + Bonds::compute_asset_reward(ASSET_ID, staked_value)
		);
	})
}

#[test]
/* fn insufficient_rewards() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;
		let staked_value = 100_000;
		assert_bond_stake(staker_id, staked_value, ASSET_ID);

		// Move era to be able to claim
		advance_to_era(2);
		let pallet_balance = Assets::balance(ASSET_ID, &Bonds::reward_pool());
		// The pallet has no founds to pay rewards
		assert_ok!(
			<Assets as fungibles::Transfer<AccountId>>::transfer(
				ASSET_ID,
				&Bonds::reward_pool(),
				&staker_id,
				pallet_balance - 1,
				true
			)
		);

		// Claim asset rewards should set the asset unactive
		assert_ok!(Bonds::claim_asset_rewards(RuntimeOrigin::signed(staker_id), ASSET_ID));

		System::assert_last_event(RuntimeEvent::Bonds(Event::StakingSttoped(ASSET_ID)));

		assert_eq!(Bonds::registered_assets(ASSET_ID).unwrap().state, AssetState::Inactive);
	})
} */

#[test]
fn cancel_unbond_is_ok() {
	ExternalityBuilder::build().execute_with(|| {
		initialize_first_block();
		register_asset(0, ASSET_ID);

		let staker_id = 1;
		let staked_value = 100;
		let unbond_amount = 10;

		assert_bond_stake(staker_id, staked_value, ASSET_ID);

		advance_to_era(2);

		assert_unbond_unstake(staker_id, unbond_amount, ASSET_ID);

		advance_to_era(3);

		assert_unbond_unstake(staker_id, unbond_amount, ASSET_ID);

		assert_ok!(Bonds::cancel_unbond(RuntimeOrigin::signed(staker_id), ASSET_ID, 1));

		let ledger = AssetLedger::<Test>::get(ASSET_ID, staker_id);

		assert_eq!(ledger.locked, staked_value);

		assert_eq!(ledger.unbonding_info.unlocking_chunks.len(), 1);

		let staker_info = AssetGeneralStakerInfo::<Test>::get(ASSET_ID, staker_id);

		assert_eq!(staker_info.latest_staked_value(), staked_value - unbond_amount);
		System::assert_last_event(RuntimeEvent::Bonds(Event::CancelUnbond(staker_id, ASSET_ID)));
	})
}
