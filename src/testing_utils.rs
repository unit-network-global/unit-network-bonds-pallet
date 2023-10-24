use super::{ pallet::pallet::Event, * };
use frame_support::assert_ok;
use mock::{ EraIndex, * };

/// Helper struct used to store information relevant to era/staker combination.
pub(crate) struct MemorySnapshot {
	asset_era_info: Option<EraInfo<Balance>>,
	asset_staker_info: StakerInfo<Balance, Moment>,
	asset_balance: Balance,
	asset_ledger: AccountLedger<Balance>,
}

impl MemorySnapshot {
	/// Prepares a new `MemorySnapshot` struct based on the given arguments.
	pub(crate) fn all(era: EraIndex, account: AccountId, asset_id: AssetId) -> Self {
		Self {
			asset_era_info: Bonds::asset_general_era_info(&asset_id, &era),
			asset_staker_info: Bonds::asset_staker_info(&asset_id, &account),
			asset_balance: Assets::balance(asset_id, &account),
			asset_ledger: Bonds::asset_ledger(&asset_id, &account),
		}
	}
}

pub(crate) fn register_asset(registrar: AccountId, asset_id: AssetId) {
	// Asset shouldn't be registered.
	assert!(!RegisteredAssets::<Test>::contains_key(asset_id));

	// Verify op is successful
	assert_ok!(
		Bonds::register_asset(RuntimeOrigin::signed(registrar), asset_id, 1_000_000, 28, ASSET_APR)
	);

	let asset_info = RegisteredAssets::<Test>::get(asset_id).unwrap();
	assert_eq!(asset_info.state, AssetState::Active);
	assert_eq!(asset_info.registrar, registrar);
}

/// Perform `bond_stake` with all the accompanied checks including before/after storage comparison.
pub(crate) fn assert_bond_stake(staker: AccountId, value: Balance, asset_id: AssetId) {
	let current_era = Bonds::current_era();
	let init_state = MemorySnapshot::all(current_era, staker, asset_id);

	// Calculate the expected Asset value that will be staked.
	let asset_available_for_staking = init_state.asset_balance;
	let asset_staking_value = asset_available_for_staking.min(value);

	// Perform staking for Asset and verify everything is as expected
	assert_ok!(Bonds::asset_bond_and_stake(RuntimeOrigin::signed(staker), ASSET_ID, value));
	System::assert_last_event(
		RuntimeEvent::Bonds(Event::AssetBondAndStake(staker, ASSET_ID, asset_staking_value))
	);

	let final_state = MemorySnapshot::all(current_era, staker, asset_id);

	// Verify the remaining states
	// Asset asserts
	assert_eq!(
		final_state.asset_era_info.as_ref().unwrap().staked,
		init_state.asset_era_info.as_ref().unwrap().staked + asset_staking_value
	);
	assert_eq!(
		final_state.asset_era_info.as_ref().unwrap().locked,
		init_state.asset_era_info.as_ref().unwrap().locked + asset_staking_value
	);
	assert_eq!(
		final_state.asset_staker_info.latest_staked_value(),
		init_state.asset_staker_info.latest_staked_value() + asset_staking_value
	);
}

/// Used to perform start_unbonding with success and storage assertions.
pub(crate) fn assert_unbond_unstake(staker: AccountId, value: Balance, asset_id: AssetId) {
	// Get latest staking info
	let current_era = Bonds::current_era();
	let init_state = MemorySnapshot::all(current_era, staker, asset_id);

	// Calculate the expected resulting unbonding amount for the asset
	let asset_remaining_staked = init_state.asset_staker_info
		.latest_staked_value()
		.saturating_sub(value);
	let expected_asset_unbond_amount = if asset_remaining_staked < MINIMUM_STAKING_AMOUNT {
		init_state.asset_staker_info.latest_staked_value()
	} else {
		value
	};

	// Ensure unstaking is successful and events are emitted
	assert_ok!(Bonds::asset_unbond_and_unstake(RuntimeOrigin::signed(staker), ASSET_ID, value));
	System::assert_last_event(
		RuntimeEvent::Bonds(
			Event::AssetUnbondAndUnstake(staker, ASSET_ID, expected_asset_unbond_amount)
		)
	);

	// Fetch the latest unbonding info so we can compare it to initial unbonding info
	let final_state = MemorySnapshot::all(current_era, staker, asset_id);

	let expected_unlock_era_asset = current_era + AssetUnbondingPeriod::<Test>::get(ASSET_ID);
	match
		init_state.asset_ledger.unbonding_info
			.vec()
			.binary_search_by(|x| x.unlock_era.cmp(&expected_unlock_era_asset))
	{
		Ok(_) =>
			assert_eq!(
				init_state.asset_ledger.unbonding_info.len(),
				final_state.asset_ledger.unbonding_info.len()
			),
		Err(_) =>
			assert_eq!(
				init_state.asset_ledger.unbonding_info.len() + 1,
				final_state.asset_ledger.unbonding_info.len()
			),
	}
	assert_eq!(
		init_state.asset_ledger.unbonding_info.sum() + expected_asset_unbond_amount,
		final_state.asset_ledger.unbonding_info.sum()
	);

	// Push the unlocking chunk we expect to have at the end and compare two structs
	let mut asset_unbonding_info = init_state.asset_ledger.unbonding_info.clone();
	asset_unbonding_info.add(UnlockingChunk {
		amount: expected_asset_unbond_amount,
		unlock_era: current_era + AssetUnbondingPeriod::<Test>::get(ASSET_ID),
	});
	assert_eq!(asset_unbonding_info, final_state.asset_ledger.unbonding_info);

	// Ensure that total locked value for staker hasn't been changed for Asset.

	assert_eq!(init_state.asset_ledger.locked, final_state.asset_ledger.locked);
	if final_state.asset_ledger.is_empty() {
		assert!(!AssetLedger::<Test>::contains_key(ASSET_ID, &staker));
	}

	assert_eq!(
		init_state.asset_staker_info.latest_staked_value() - expected_asset_unbond_amount,
		final_state.asset_staker_info.latest_staked_value()
	);

	// Ensure that total staked value has been decreased
	assert_eq!(
		init_state.asset_era_info.as_ref().unwrap().staked - expected_asset_unbond_amount,
		final_state.asset_era_info.as_ref().unwrap().staked
	);

	// Ensure that locked amount is the same since this will only start the unbonding period
	assert_eq!(
		init_state.asset_era_info.as_ref().unwrap().locked,
		final_state.asset_era_info.as_ref().unwrap().locked
	);
}

/// Used to perform start_unbonding with success and storage assertions.
pub(crate) fn assert_withdraw_unbonded(staker: AccountId) {
	let current_era = Bonds::current_era();

	let init_asset_era_info = AssetGeneralEraInfo::<Test>::get(ASSET_ID, current_era).unwrap();
	let init_asset_ledger = AssetLedger::<Test>::get(ASSET_ID, &staker);

	// Get the current unlocking chunks
	let (asset_valid_info, asset_remaining_info) =
		init_asset_ledger.unbonding_info.partition(current_era);
	let expected_asset_unbond_amount = asset_valid_info.sum();

	// Ensure op is successful and event is emitted
	assert_ok!(Bonds::withdraw_unbonded_asset(RuntimeOrigin::signed(staker), ASSET_ID));
	System::assert_last_event(
		RuntimeEvent::Bonds(
			Event::WithdrawnFromAsset(staker, ASSET_ID, expected_asset_unbond_amount)
		)
	);

	// Fetch the latest unbonding info so we can compare it to expected remainder
	let final_asset_ledger = AssetLedger::<Test>::get(ASSET_ID, &staker);
	assert_eq!(asset_remaining_info, final_asset_ledger.unbonding_info);
	if final_asset_ledger.unbonding_info.is_empty() && final_asset_ledger.locked == 0 {
		assert!(!AssetLedger::<Test>::contains_key(ASSET_ID, &staker));
	}

	// Compare the ledger and total staked value

	let final_asset_stakes = AssetGeneralEraInfo::<Test>::get(ASSET_ID, current_era).unwrap();
	assert_eq!(final_asset_stakes.staked, init_asset_era_info.staked);
	assert_eq!(
		final_asset_stakes.locked,
		init_asset_era_info.locked - expected_asset_unbond_amount
	);
	assert_eq!(final_asset_ledger.locked, init_asset_ledger.locked - expected_asset_unbond_amount);
}

/// Used to perform claim for stakers with success assertion
pub(crate) fn assert_claim_staking_reward(claimer: AccountId) {
	let current_era = Bonds::current_era();
	let (claim_era, _) = Bonds::asset_staker_info(ASSET_ID, &claimer).claim(current_era, 0u64);

	//clean up possible leftover events
	System::reset_events();

	let init_state_claim_era = MemorySnapshot::all(claim_era, claimer, ASSET_ID);
	let init_state_current_era = MemorySnapshot::all(current_era, claimer, ASSET_ID);

	let (claim_asset_era, asset_staked) = init_state_claim_era.asset_staker_info
		.clone()
		.claim(current_era, 0u64);
	assert!(claim_asset_era > 0); // Sanity check - if this fails, method is being used incorrectly

	let calculated_asset_reward = Bonds::compute_asset_reward(ASSET_ID, asset_staked);

	assert_ok!(Bonds::claim_asset_rewards(RuntimeOrigin::signed(claimer), ASSET_ID));

	let final_state_current_era = MemorySnapshot::all(current_era, claimer, ASSET_ID);

	// assert staked and free balances depending on restake check,
	assert_restake_reward(
		&init_state_current_era,
		&final_state_current_era,
		calculated_asset_reward
	);

	// check for stake event if restaking is performed
	if
		Bonds::should_restake_asset_reward(
			init_state_current_era.asset_ledger.reward_destination,
			init_state_current_era.asset_staker_info.latest_staked_value()
		)
	{
		// In this case, the last two event should be for claiming assets.
		let events = events();
		let second_last_event = &events[events.len() - 2];
		let last_event = &events[events.len() - 1];
		assert_eq!(
			second_last_event.clone(),
			Event::<Test>::AssetBondAndStake(claimer, ASSET_ID, calculated_asset_reward)
		);
		assert_eq!(
			last_event.clone(),
			Event::<Test>::AssetReward(claimer, ASSET_ID, claim_asset_era, calculated_asset_reward)
		);
	}

	let (new_era, _) = final_state_current_era.asset_staker_info.clone().claim(current_era, 0u64);

	assert!(new_era.is_zero() || new_era > claim_era);

	if final_state_current_era.asset_staker_info.is_empty() {
		assert!(new_era.is_zero());
		assert!(!AssetGeneralStakerInfo::<Test>::contains_key(ASSET_ID, &claimer));
	}

	// Old `claim_era` info should never be changed
	let final_state_claim_era = MemorySnapshot::all(claim_era, claimer, ASSET_ID);
	assert_eq!(
		init_state_claim_era.asset_era_info.unwrap(),
		final_state_claim_era.asset_era_info.unwrap()
	);
}

// assert staked and locked states depending on should_restake_reward
// returns should_restake_reward result so further checks can be made
fn assert_restake_reward(
	init_state_current_era: &MemorySnapshot,
	final_state_current_era: &MemorySnapshot,
	reward_asset: Balance
) {
	if
		Bonds::should_restake_asset_reward(
			init_state_current_era.asset_ledger.reward_destination,
			init_state_current_era.asset_staker_info.latest_staked_value()
		)
	{
		// staked values should increase
		assert_eq!(
			init_state_current_era.asset_staker_info.latest_staked_value() + reward_asset,
			final_state_current_era.asset_staker_info.latest_staked_value()
		);
		assert_eq!(
			init_state_current_era.asset_era_info.as_ref().unwrap().staked + reward_asset,
			final_state_current_era.asset_era_info.as_ref().unwrap().staked
		);
		assert_eq!(
			init_state_current_era.asset_era_info.as_ref().unwrap().locked + reward_asset,
			final_state_current_era.asset_era_info.as_ref().unwrap().locked
		);
	} else {
		// staked values should remain the same, and free balance increase
		assert_eq!(
			init_state_current_era.asset_balance + reward_asset,
			final_state_current_era.asset_balance
		);
		assert_eq!(
			init_state_current_era.asset_era_info.as_ref().unwrap().staked,
			final_state_current_era.asset_era_info.as_ref().unwrap().staked
		);
		assert_eq!(
			init_state_current_era.asset_era_info.as_ref().unwrap().locked,
			final_state_current_era.asset_era_info.as_ref().unwrap().locked
		);
	}
}
