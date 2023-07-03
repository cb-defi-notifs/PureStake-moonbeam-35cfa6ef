// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

// We want to avoid including the rococo-runtime here.
// TODO: whenever a conclusion is taken from https://github.com/paritytech/substrate/issues/8158

use crate::stake_calls::{StakeCallV0, StakeCallV1};
use cumulus_primitives_core::{relay_chain::HrmpChannelId, ParaId};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::traits::{AccountIdLookup, StaticLookup};
use sp_runtime::AccountId32;
use sp_std::vec::Vec;

#[derive(Encode, Decode)]
pub enum RelayCallV0 {
	#[codec(index = 7u8)]
	Stake(StakeCallV0),
}

#[derive(Encode, Decode)]
pub enum RelayCallV1 {
	#[codec(index = 26u8)]
	// the index should match the position of the module in `construct_runtime!`
	Utility(UtilityCall),

	#[codec(index = 7u8)]
	Stake(StakeCallV1),
	#[codec(index = 60u8)]
	// the index should match the position of the module in `construct_runtime!`
	Hrmp(HrmpCall),
}

// Utility call encoding, needed for xcm transactor pallet
#[derive(Encode, Decode)]
pub enum UtilityCall {
	#[codec(index = 1u8)]
	AsDerivative(u16),
}

// HRMP call encoding, needed for xcm transactor pallet
#[derive(Encode, Decode)]
pub enum HrmpCall {
	#[codec(index = 0u8)]
	InitOpenChannel(ParaId, u32, u32),
	#[codec(index = 1u8)]
	AcceptOpenChannel(ParaId),
	#[codec(index = 2u8)]
	CloseChannel(HrmpChannelId),
	#[codec(index = 6u8)]
	CancelOpenRequest(HrmpChannelId, u32),
}

pub struct PolkadotEncoder;

impl xcm_primitives::UtilityEncodeCall for PolkadotEncoder {
	fn encode_call(self, call: xcm_primitives::UtilityAvailableCalls) -> Vec<u8> {
		match call {
			xcm_primitives::UtilityAvailableCalls::AsDerivative(a, b) => {
				let mut call = RelayCallV1::Utility(UtilityCall::AsDerivative(a.clone())).encode();
				// If we encode directly we inject the call length,
				// so we just append the inner call after encoding the outer
				call.append(&mut b.clone());
				call
			}
		}
	}
}

impl xcm_primitives::HrmpEncodeCall for PolkadotEncoder {
	fn hrmp_encode_call(
		call: xcm_primitives::HrmpAvailableCalls,
	) -> Result<Vec<u8>, xcm::latest::Error> {
		match call {
			xcm_primitives::HrmpAvailableCalls::InitOpenChannel(a, b, c) => Ok(RelayCallV1::Hrmp(
				HrmpCall::InitOpenChannel(a.clone(), b.clone(), c.clone()),
			)
			.encode()),
			xcm_primitives::HrmpAvailableCalls::AcceptOpenChannel(a) => {
				Ok(RelayCallV1::Hrmp(HrmpCall::AcceptOpenChannel(a.clone())).encode())
			}
			xcm_primitives::HrmpAvailableCalls::CloseChannel(a) => {
				Ok(RelayCallV1::Hrmp(HrmpCall::CloseChannel(a.clone())).encode())
			}
			xcm_primitives::HrmpAvailableCalls::CancelOpenRequest(a, b) => {
				Ok(RelayCallV1::Hrmp(HrmpCall::CancelOpenRequest(a.clone(), b.clone())).encode())
			}
		}
	}
}

impl pallet_evm_precompile_relay_encoder::StakeEncodeCall for PolkadotEncoder {
	fn encode_call(call: pallet_evm_precompile_relay_encoder::AvailableStakeCalls) -> Vec<u8> {
		match call {
			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Bond(a, b, c) => match a {
				None => RelayCallV1::Stake(StakeCallV1::Bond(b, c)).encode(),
				Some(i) => RelayCallV0::Stake(StakeCallV0::Bond(i.into(), b, c)).encode(),
			},

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::BondExtra(a) => {
				RelayCallV1::Stake(StakeCallV1::BondExtra(a)).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Unbond(a) => {
				RelayCallV1::Stake(StakeCallV1::Unbond(a)).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::WithdrawUnbonded(a) => {
				RelayCallV1::Stake(StakeCallV1::WithdrawUnbonded(a)).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Validate(a) => {
				RelayCallV1::Stake(StakeCallV1::Validate(a)).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Chill => {
				RelayCallV1::Stake(StakeCallV1::Chill).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::SetPayee(a) => {
				RelayCallV1::Stake(StakeCallV1::SetPayee(a.into())).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::SetController(a) => match a {
				None => RelayCallV1::Stake(StakeCallV1::SetController).encode(),
				Some(i) => RelayCallV0::Stake(StakeCallV0::SetController(i.into())).encode(),
			},

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Rebond(a) => {
				RelayCallV1::Stake(StakeCallV1::Rebond(a.into())).encode()
			}

			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Nominate(a) => {
				let nominated: Vec<<AccountIdLookup<AccountId32, ()> as StaticLookup>::Source> =
					a.iter().map(|add| (*add).clone().into()).collect();

				RelayCallV1::Stake(StakeCallV1::Nominate(nominated)).encode()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::polkadot::PolkadotEncoder;
	use frame_support::traits::PalletInfo;
	use pallet_evm_precompile_relay_encoder::StakeEncodeCall;
	use sp_runtime::Perbill;

	#[test]
	fn test_as_derivative() {
		let mut expected_encoded: Vec<u8> = Vec::new();
		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Utility,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_utility::Call::<polkadot_runtime::Runtime>::as_derivative {
			index: 1,
			call: polkadot_runtime::RuntimeCall::Staking(pallet_staking::Call::<
				polkadot_runtime::Runtime,
			>::chill {})
			.into(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		let call_bytes = <PolkadotEncoder as StakeEncodeCall>::encode_call(
			pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Chill,
		);

		expected_encoded.append(&mut expected);

		assert_eq!(
			xcm_primitives::UtilityEncodeCall::encode_call(
				PolkadotEncoder,
				xcm_primitives::UtilityAvailableCalls::AsDerivative(1, call_bytes)
			),
			expected_encoded
		);
	}

	#[test]
	fn test_stake_bond() {
		let mut expected_encoded: Vec<u8> = Vec::new();
		let relay_account: AccountId32 = [1u8; 32].into();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::bond {
			controller: relay_account.clone().into(),
			value: 100u32.into(),
			payee: pallet_staking::RewardDestination::Controller,
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Bond(
					relay_account.into(),
					100u32.into(),
					pallet_staking::RewardDestination::Controller
				)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_bond_extra() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::bond_extra {
			max_additional: 100u32.into(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::BondExtra(100u32.into(),)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_unbond() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::unbond {
			value: 100u32.into(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Unbond(100u32.into(),)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_withdraw_unbonded() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::withdraw_unbonded {
			num_slashing_spans: 100u32,
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::WithdrawUnbonded(100u32,)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_validate() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let validator_prefs = pallet_staking::ValidatorPrefs {
			commission: Perbill::from_percent(5),
			blocked: true,
		};

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::validate {
			prefs: validator_prefs.clone(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Validate(validator_prefs)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_nominate() {
		let mut expected_encoded: Vec<u8> = Vec::new();
		let relay_account: AccountId32 = [1u8; 32].into();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::nominate {
			targets: vec![relay_account.clone().into()],
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Nominate(vec![
					relay_account.into()
				])
			),
			expected_encoded
		);
	}
	#[test]
	fn test_stake_chill() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::chill {}.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Chill
			),
			expected_encoded
		);
	}

	#[test]
	fn test_set_payee() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::set_payee {
			payee: pallet_staking::RewardDestination::Controller,
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::SetPayee(
					pallet_staking::RewardDestination::Controller
				)
			),
			expected_encoded
		);
	}

	#[test]
	fn test_set_controller() {
		let mut expected_encoded: Vec<u8> = Vec::new();
		let relay_account: AccountId32 = [1u8; 32].into();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::set_controller {
			controller: relay_account.clone().into(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::SetController(
					relay_account.clone().into()
				)
			),
			expected_encoded
		);
	}
	#[test]
	fn test_rebond() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Staking,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = pallet_staking::Call::<polkadot_runtime::Runtime>::rebond {
			value: 100u32.into(),
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as StakeEncodeCall>::encode_call(
				pallet_evm_precompile_relay_encoder::AvailableStakeCalls::Rebond(100u32.into())
			),
			expected_encoded
		);
	}

	#[test]
	fn test_hrmp_init() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Hrmp,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = polkadot_runtime_parachains::hrmp::Call::<
			polkadot_runtime::Runtime
		>::hrmp_init_open_channel {
			recipient: 1000u32.into(),
			proposed_max_capacity: 100u32,
			proposed_max_message_size: 100u32,
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as xcm_primitives::HrmpEncodeCall>::hrmp_encode_call(
				xcm_primitives::HrmpAvailableCalls::InitOpenChannel(
					1000u32.into(),
					100u32.into(),
					100u32.into()
				)
			),
			Ok(expected_encoded)
		);
	}

	#[test]
	fn test_hrmp_accept() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Hrmp,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = polkadot_runtime_parachains::hrmp::Call::<
			polkadot_runtime::Runtime
		>::hrmp_accept_open_channel {
			sender: 1000u32.into()
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as xcm_primitives::HrmpEncodeCall>::hrmp_encode_call(
				xcm_primitives::HrmpAvailableCalls::AcceptOpenChannel(1000u32.into(),)
			),
			Ok(expected_encoded)
		);
	}

	#[test]
	fn test_hrmp_close() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Hrmp,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let mut expected = polkadot_runtime_parachains::hrmp::Call::<
			polkadot_runtime::Runtime
		>::hrmp_close_channel {
			channel_id: HrmpChannelId {
				sender: 1000u32.into(),
				recipient: 1001u32.into()
			}
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as xcm_primitives::HrmpEncodeCall>::hrmp_encode_call(
				xcm_primitives::HrmpAvailableCalls::CloseChannel(HrmpChannelId {
					sender: 1000u32.into(),
					recipient: 1001u32.into()
				})
			),
			Ok(expected_encoded)
		);
	}

	#[test]
	fn test_hrmp_cancel() {
		let mut expected_encoded: Vec<u8> = Vec::new();

		let index = <polkadot_runtime::Runtime as frame_system::Config>::PalletInfo::index::<
			polkadot_runtime::Hrmp,
		>()
		.unwrap() as u8;
		expected_encoded.push(index);

		let channel_id = HrmpChannelId {
			sender: 1u32.into(),
			recipient: 1u32.into(),
		};
		let open_requests: u32 = 1;

		let mut expected = polkadot_runtime_parachains::hrmp::Call::<
			polkadot_runtime::Runtime
		>::hrmp_cancel_open_request {
			channel_id: channel_id.clone(),
			open_requests
		}
		.encode();
		expected_encoded.append(&mut expected);

		assert_eq!(
			<PolkadotEncoder as xcm_primitives::HrmpEncodeCall>::hrmp_encode_call(
				xcm_primitives::HrmpAvailableCalls::CancelOpenRequest(
					channel_id.clone(),
					open_requests
				)
			),
			Ok(expected_encoded)
		);
	}
}
