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


//! Autogenerated weights for pallet_precompile_benchmarks
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Ahmads-MBP`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_precompile_benchmarks
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_precompile_benchmarks.
pub trait WeightInfo {
	fn verify_entry(x: u32, ) -> Weight;
	fn latest_relay_block() -> Weight;
}

/// Weights for pallet_precompile_benchmarks using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `x` is `[100, 2000]`.
	fn verify_entry(x: u32, ) -> Weight {
		// Minimum execution time: 97_000_000 picoseconds.
		Weight::from_parts(98_000_000, 0)
			// Standard Error: 2_811
			.saturating_add(Weight::from_parts(813_171, 0).saturating_mul(x.into()))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRootKeys` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRootKeys` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	fn latest_relay_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `1526`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 1526)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `RelayStorageRoots::RelayStorageRoot` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[100, 2000]`.
	fn verify_entry(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110`
		//  Estimated: `3509`
		// Minimum execution time: 97_000_000 picoseconds.
		Weight::from_parts(98_000_000, 3509)
			// Standard Error: 2_811
			.saturating_add(Weight::from_parts(813_171, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRootKeys` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRootKeys` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	fn latest_relay_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `1526`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 1526)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}