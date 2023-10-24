//! # Unit Bonds Pallet
//!
//! ## Overview
//!
//! Pallet that implements staking assets for Unit Network.
//!
//! This is used to stake assets for the purpose of earning rewards.
//! The earning for stakers is calculated based on the APR (Annual Percentage Rate) configured.
//!
//! Rewards can be claimed after an era and when the era finishes, the stakers can claim their rewards for that era.
//! Only the last 7 era rewards can be claimed.
//!
//! Reward claiming isn't automated since the whole process is done on-chain.
//! Stakers are responsibles for claiming their own rewards.
//! 
//! ### Goals
//!
//! The pallet is designed to make the following possible:
//!
//! * Register an asset for staking by the asset´s owner.
//! * Unregister an asset of staking by the asset´s owner.
//! * Stake funds to earn rewards.
//! * Start unbonding period for staked assets and withdraw assets that already finish the process.
//! * Cancel an unbonding period a stake back the funds.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! - `asset_bond_and_stake`: Transfer and lock stakers tokens and update the staking info.
//! - `asset_unbond_and_unstake`: Removes staked tokens, starting the unbonding process for the unstaked funds.
//! - `withdraw_unbonded_asset`: Withdraws all funds that have completed the unbonding period.
//! - `claim_asset_rewards`: Claims staker reward for the last claimable era.
//! - `cancel_unbond`: Cancel the unbonding process for tokens and stake them back.
//!
//!	### Privileged Functions
//!
//! - `register_asset`: Register an asset by giving tokens to the pool and setting stake parameters, the sender needs to reserve an amount of Unit.
//! - `unregister_asset`: Unregister an asset and getting back the reserved Unit.
//!
//! ### Other
//!
//! - `on_initialize` - part of `Hooks` trait, it's important to call this per block since it handles era advancement.
//! - `account_id` - returns pallet's account Id
//! - `reward_pool - returns rewards pool´s account Id
//! - `compute_asset_reward` - used to compute the reward for an amount of tokens.
//!
//!//! Please refer to the [`Call`] enum and its associated variants for documentation on each
//! function.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{ Decode, Encode, HasCompact };
use frame_support::traits::Currency;
use frame_system::{ self as system };
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::{ traits::{ AtLeast32BitUnsigned, Zero }, RuntimeDebug };
use sp_std::{ ops::Add, prelude::*, vec };

pub mod pallet;

#[cfg(any(feature = "runtime-benchmarks"))]
pub mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod testing_utils;
#[cfg(test)]
mod tests;

pub use pallet::pallet::*;

pub mod weights;
//pub use weights::WeightInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;

/// Indicates if the staking is active for an asset
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum AssetState {
	/// Asset is registered and staking is active.
	Active,
	/// The asset is registered but staking is not active.
	Inactive,
}

/// Information about the status and registration of an asset.
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AssetInfo<AccountId> {
	/// Account that registered the asset
	registrar: AccountId,
	/// Current Asset State
	state: AssetState,
}

/// Contains information about account's locked & unbonding balances.
#[derive(Clone, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct AccountLedger<Balance: AtLeast32BitUnsigned + Default + Copy> {
	/// Total balance locked.
	#[codec(compact)]
	pub locked: Balance,
	/// Information about unbonding chunks.
	unbonding_info: UnbondingInfo<Balance>,
	/// Instruction on how to handle reward payout
	reward_destination: RewardDestination,
}

impl<Balance: AtLeast32BitUnsigned + Default + Copy> AccountLedger<Balance> {
	/// `true` if ledger is empty (no locked funds, no unbonding chunks), `false` otherwise.
	pub fn is_empty(&self) -> bool {
		self.locked.is_zero() && self.unbonding_info.is_empty()
	}
}

/// A record for total amount staked and locked for an era
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct EraInfo<Balance: HasCompact> {
	/// Total staked amount in an era
	#[codec(compact)]
	pub staked: Balance,
	/// Total locked amount in an era
	#[codec(compact)]
	pub locked: Balance,
}

/// Used to represent how much was staked in a particular era.
/// E.g. `{staked: 1000, era: 5}` means that in era `5`, staked amount was 1000.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct EraStake<Balance: AtLeast32BitUnsigned + Copy, Moment> {
	/// Staked amount in era
	#[codec(compact)]
	staked: Balance,
	/// Staked era
	#[codec(compact)]
	era: EraIndex,
	/// When was staked
	timestamp: Moment,
}

impl<Balance: AtLeast32BitUnsigned + Copy, Moment> EraStake<Balance, Moment> {
	/// Create a new instance of `EraStake` with given values
	fn new(staked: Balance, era: EraIndex, timestamp: Moment) -> Self {
		Self { staked, era, timestamp }
	}
}

/// Mode of era-forcing.
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum Forcing {
	/// Not forcing anything - just let whatever happen.
	NotForcing,
	/// Force a new era, then reset to `NotForcing` as soon as it is done.
	/// Note that this will force to trigger an election until a new era is triggered, if the
	/// election failed, the next session end will trigger a new election again, until success.
	ForceNew,
}

/// Instruction on how to handle reward payout for stakers.
/// In order to make staking more competitive, majority of stakers will want to
/// automatically restake anything they earn.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum RewardDestination {
	/// Rewards are transferred to stakers free balance without any further action.
	FreeBalance,
	/// Rewards are transferred to stakers balance and are immediately re-staked.
	StakeBalance,
}

impl Default for RewardDestination {
	fn default() -> Self {
		RewardDestination::StakeBalance
	}
}

/// Used to provide a compact and bounded storage for information about stakes in unclaimed eras.
///
/// In order to avoid creating a separate storage entry for each `(staker, era)` tuple,
/// this struct is used to provide a more memory efficient solution.
///
/// Basic idea is to store `EraStake` structs into a vector from which a complete
/// picture of **unclaimed eras** and stakes can be constructed.
///
/// # Example
/// For simplicity, the following example will represent `EraStake` using `<era, stake>` notation.
/// Let us assume we have the following vector in `StakerInfo` struct.
///
/// `[<5, 1000>, <6, 1500>, <8, 2100>, <9, 0>, <11, 500>]`
///
/// This tells us which eras are unclaimed and how much it was staked in each era.
/// The interpretation is the following:
/// 1. In era **5**, staked amount was **1000** (interpreted from `<5, 1000>`)
/// 2. In era **6**, staker staked additional **500**, increasing total staked amount to **1500**
/// 3. No entry for era **7** exists which means there were no changes from the former entry.
///    This means that in era **7**, staked amount was also **1500**
/// 4. In era **8**, staker staked an additional **600**, increasing total stake to **2100**
/// 5. In era **9**, staker unstaked everything (interpreted from `<9, 0>`)
/// 6. No changes were made in era **10** so we can interpret this same as the previous entry which means **0** staked amount.
/// 7. In era **11**, staker staked **500**, making his stake active again after 2 eras of inactivity.
///
/// **NOTE:** It is important to understand that staker **DID NOT** claim any ts during this period.
///
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct StakerInfo<Balance: AtLeast32BitUnsigned + Copy, Moment: Copy> {
	// Size of this list would be limited by a configurable constant
	stakes: Vec<EraStake<Balance, Moment>>,
}

impl<Balance: AtLeast32BitUnsigned + Copy, Moment: Copy> StakerInfo<Balance, Moment> {
	/// `true` if no active stakes and unclaimed eras exist, `false` otherwise
	fn is_empty(&self) -> bool {
		self.stakes.is_empty()
	}

	/// number of `EraStake` chunks
	fn len(&self) -> u32 {
		self.stakes.len() as u32
	}

	/// Stakes some value in the specified era.
	///
	/// User should ensure that given era is either equal or greater than the
	/// latest available era in the staking info.
	///
	/// # Example
	///
	/// The following example demonstrates how internal vector changes when `stake` is called:
	///
	/// `stakes: [<5, 1000>, <7, 1300>]`
	/// * `stake(7, 100)` will result in `[<5, 1000>, <7, 1400>]`
	/// * `stake(9, 200)` will result in `[<5, 1000>, <7, 1400>, <9, 1600>]`
	///
	fn stake(&mut self, current_era: EraIndex, value: Balance, now: Moment) -> Result<(), &str> {
		if let Some(era_stake) = self.stakes.last_mut() {
			if era_stake.era > current_era {
				return Err("Unexpected era".into());
			}

			let new_stake_value = era_stake.staked.saturating_add(value);

			if current_era == era_stake.era {
				*era_stake = EraStake::new(new_stake_value, current_era, now);
			} else {
				self.stakes.push(EraStake::new(new_stake_value, current_era, now));
			}
		} else {
			self.stakes.push(EraStake::new(value, current_era, now));
		}

		Ok(())
	}

	/// Unstakes some value in the specified era.
	///
	/// User should ensure that given era is either equal or greater than the
	/// latest available era in the staking info.
	///
	/// # Example 1
	///
	/// `stakes: [<5, 1000>, <7, 1300>]`
	/// * `unstake(7, 100)` will result in `[<5, 1000>, <7, 1200>]`
	/// * `unstake(9, 400)` will result in `[<5, 1000>, <7, 1200>, <9, 800>]`
	/// * `unstake(10, 800)` will result in `[<5, 1000>, <7, 1200>, <9, 800>, <10, 0>]`
	///
	/// # Example 2
	///
	/// `stakes: [<5, 1000>]`
	/// * `unstake(5, 1000)` will result in `[]`
	///
	/// Note that if no unclaimed eras remain, vector will be cleared.
	///
	fn unstake(&mut self, current_era: EraIndex, value: Balance, now: Moment) -> Result<(), &str> {
		if let Some(era_stake) = self.stakes.last_mut() {
			if era_stake.era > current_era {
				return Err("Unexpected era".into());
			}

			let new_stake_value = era_stake.staked.saturating_sub(value);
			if current_era == era_stake.era {
				*era_stake = EraStake::new(new_stake_value, current_era, now);
			} else {
				self.stakes.push(EraStake::new(new_stake_value, current_era, now));
			}

			// Removes unstaked values if they're no longer valid for comprehension
			if !self.stakes.is_empty() && self.stakes[0].staked.is_zero() {
				self.stakes.remove(0);
			}
		}

		Ok(())
	}

	/// `Claims` the oldest era available for claiming. The oldest era for claim should be no more than 7 eras behind current era.
	/// In case valid era exists, returns `(claim era, staked amount)` tuple.
	/// If no valid era exists, returns `(0, 0)` tuple.
	///
	/// # Example
	///
	/// The following example will demonstrate how the internal vec changes when `claim` is called consecutively.
	///
	/// `stakes: [<5, 1000>, <7, 1300>, <8, 0>, <9, 150>, <13, 2000>, <15, 3000>]`
	///
	/// 1. `claim(era)` will return `(9, 150)`
	///     Internal vector is modified to `[ <13, 2000>, <15, 3000>]`
	///
	/// 2. `claim(era)` will return `(13, 2000)`.
	///    Internal vector is modified to `[<14, 2000>, <15, 3000>]`
	///
	/// 3. `claim(era)` will return `(14, 2000)`.
	///    Internal vector is modified to `[<15, 3000>]`
	///    Note that `0` staked period is discarded since nothing can be claimed there.
	///
	/// 4. `claim(era)` will return `(15, 3000)`.
	///    Internal vector is modified to `[16, 3000]`
	///
	/// Repeated calls would continue to modify vector following the same rule as in *4.*
	///
	fn claim(&mut self, current_era: EraIndex, now: Moment) -> (EraIndex, Balance) {
		let mut exit = false;
		let mut era_claimed = (0, Zero::zero());
		while !exit {
			if let Some(era_stake) = self.stakes.first() {
				let era_stake = *era_stake;

				if self.stakes.len() == 1 || self.stakes[1].era > era_stake.era + 1 {
					self.stakes[0] = EraStake {
						staked: era_stake.staked,
						era: era_stake.era.saturating_add(1),
						timestamp: now,
					};
				} else {
					// in case: self.stakes[1].era == era_stake.era + 1
					self.stakes.remove(0);
				}

				// Removes unstaked values if they're no longer valid for comprehension
				if !self.stakes.is_empty() && self.stakes[0].staked.is_zero() {
					self.stakes.remove(0);
				}

				if era_stake.era < current_era.saturating_sub(7) {
					continue;
				}

				exit = true;
				era_claimed = (era_stake.era, era_stake.staked);
			} else {
				exit = true;
			}
		}
		era_claimed
	}

	/// Latest staked value.
	/// E.g. if staker is fully unstaked, this will return `Zero`.
	/// Otherwise returns a non-zero balance.
	pub fn latest_staked_value(&self) -> Balance {
		self.stakes.last().map_or(Zero::zero(), |x| x.staked)
	}
}

/// Represents an balance amount undergoing the unbonding process.
/// Since unbonding takes time, it's important to keep track of when and how much was unbonded.
#[derive(Clone, Copy, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct UnlockingChunk<Balance> {
	/// Amount being unlocked
	#[codec(compact)]
	amount: Balance,
	/// Era in which the amount will become unlocked and can be withdrawn.
	#[codec(compact)]
	unlock_era: EraIndex,
}

impl<Balance> UnlockingChunk<Balance> where Balance: Add<Output = Balance> + Copy {
	// Adds the specified amount to this chunk
	fn add_amount(&mut self, amount: Balance) {
		self.amount = self.amount + amount;
	}
}

/// Contains unlocking chunks.
/// This is a convenience struct that provides various utility methods to help with unbonding handling.
#[derive(Clone, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct UnbondingInfo<Balance: AtLeast32BitUnsigned + Default + Copy> {
	// Vector of unlocking chunks. Sorted in ascending order in respect to unlock_era.
	unlocking_chunks: Vec<UnlockingChunk<Balance>>,
}

impl<Balance> UnbondingInfo<Balance> where Balance: AtLeast32BitUnsigned + Default + Copy {
	/// Returns total number of unlocking chunks.
	fn len(&self) -> u32 {
		self.unlocking_chunks.len() as u32
	}

	/// True if no unlocking chunks exist, false otherwise.
	fn is_empty(&self) -> bool {
		self.unlocking_chunks.is_empty()
	}

	/// Returns sum of all unlocking chunks.
	fn sum(&self) -> Balance {
		self.unlocking_chunks
			.iter()
			.map(|chunk| chunk.amount)
			.reduce(|c1, c2| c1 + c2)
			.unwrap_or_default()
	}

	/// Adds a new unlocking chunk to the vector, preserving the unlock_era based ordering.
	fn add(&mut self, chunk: UnlockingChunk<Balance>) {
		// It is possible that the unbonding period changes so we need to account for that
		match self.unlocking_chunks.binary_search_by(|x| x.unlock_era.cmp(&chunk.unlock_era)) {
			// Merge with existing chunk if unlock_eras match
			Ok(pos) => self.unlocking_chunks[pos].add_amount(chunk.amount),
			// Otherwise insert where it should go. Note that this will in almost all cases return the last index.
			Err(pos) => self.unlocking_chunks.insert(pos, chunk),
		}
	}

	/// Partitions the unlocking chunks into two groups:
	///
	/// First group includes all chunks which have unlock era lesser or equal to the specified era.
	/// Second group includes all the rest.
	///
	/// Order of chunks is preserved in the two new structs.
	fn partition(self, era: EraIndex) -> (Self, Self) {
		let (matching_chunks, other_chunks): (
			Vec<UnlockingChunk<Balance>>,
			Vec<UnlockingChunk<Balance>>,
		) = self.unlocking_chunks.iter().partition(|chunk| chunk.unlock_era <= era);

		(
			Self {
				unlocking_chunks: matching_chunks,
			},
			Self {
				unlocking_chunks: other_chunks,
			},
		)
	}

	#[cfg(test)]
	/// Return clone of the internal vector. Should only be used for testing.
	fn vec(&self) -> Vec<UnlockingChunk<Balance>> {
		self.unlocking_chunks.clone()
	}
}

#[derive(Clone, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct RewardData<Balance, BlockNumber, AccountId, Moment> {
	// when the reward was claimed. TODO: use timestamp
	time: BlockNumber,
	amount: Balance,
	era: EraIndex,
	account: AccountId,
	timestamp: Moment,
}

/// Indicates the action that a user has made
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Action {
	/// User has staked
	Stake,
	/// User start the unbonding period
	Unstake,
	/// User cancel unbonding period and stake again
	CancelUnbond,
	/// User withdraw tokens that has completed the unbonding period
	WithdrawUnbonded,
	/// User claimed rewards
	RewardsClaimed,
}

#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ActionInfo<Balance, BlockNumber, Moment> {
	action: Action,
	amount: Balance,
	time: BlockNumber,
	index: u32,
	timestamp: Moment,
}
