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


//! Autogenerated weights for pallet_author_mapping
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-36`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// /home/ubuntu/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_author_mapping.
pub trait WeightInfo {
	fn add_association() -> Weight;
	fn update_association() -> Weight;
	fn clear_association() -> Weight;
	fn remove_keys() -> Weight;
	fn set_keys() -> Weight;
}

/// Weights for pallet_author_mapping using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn add_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `7798`
		// Minimum execution time: 40_939_000 picoseconds.
		Weight::from_parts(42_096_000, 7798)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn update_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `6590`
		// Minimum execution time: 30_561_000 picoseconds.
		Weight::from_parts(31_122_000, 6590)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn clear_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453`
		//  Estimated: `7952`
		// Minimum execution time: 41_884_000 picoseconds.
		Weight::from_parts(43_323_000, 7952)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping NimbusLookup (r:1 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn remove_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `547`
		//  Estimated: `11605`
		// Minimum execution time: 46_219_000 picoseconds.
		Weight::from_parts(47_508_000, 11605)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping NimbusLookup (r:1 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `11375`
		// Minimum execution time: 44_053_000 picoseconds.
		Weight::from_parts(45_059_000, 11375)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn add_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `7798`
		// Minimum execution time: 40_939_000 picoseconds.
		Weight::from_parts(42_096_000, 7798)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn update_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `6590`
		// Minimum execution time: 30_561_000 picoseconds.
		Weight::from_parts(31_122_000, 6590)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AuthorMapping NimbusLookup (r:0 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	fn clear_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453`
		//  Estimated: `7952`
		// Minimum execution time: 41_884_000 picoseconds.
		Weight::from_parts(43_323_000, 7952)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping NimbusLookup (r:1 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn remove_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `547`
		//  Estimated: `11605`
		// Minimum execution time: 46_219_000 picoseconds.
		Weight::from_parts(47_508_000, 11605)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: AuthorMapping NimbusLookup (r:1 w:1)
	/// Proof Skipped: AuthorMapping NimbusLookup (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	/// Proof Skipped: AuthorMapping MappingWithDeposit (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `11375`
		// Minimum execution time: 44_053_000 picoseconds.
		Weight::from_parts(45_059_000, 11375)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}