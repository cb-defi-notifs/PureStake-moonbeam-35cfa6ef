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

//! Autogenerated weights for pallet_moonbeam_orbiters
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_moonbeam_orbiters
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof
// --json-file
// raw.json
// --output
// ./benchmarks/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_moonbeam_orbiters.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight;
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight;
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight;
	#[rustfmt::skip]
	fn orbiter_register() -> Weight;
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight;
	#[rustfmt::skip]
	fn add_collator() -> Weight;
	#[rustfmt::skip]
	fn remove_collator() -> Weight;
}

/// Weights for pallet_moonbeam_orbiters using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:0)
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight {
		(31_392_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight {
		(26_451_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight {
		(26_463_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters MinOrbiterDeposit (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_register() -> Weight {
		(32_506_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:0)
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight {
		(33_919_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((6_981_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn add_collator() -> Weight {
		(12_035_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:9)
	#[rustfmt::skip]
	fn remove_collator() -> Weight {
		(25_438_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:0)
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight {
		(31_392_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight {
		(26_451_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight {
		(26_463_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: MoonbeamOrbiters MinOrbiterDeposit (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_register() -> Weight {
		(32_506_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:0)
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight {
		(33_919_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((6_981_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn add_collator() -> Weight {
		(12_035_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:9)
	#[rustfmt::skip]
	fn remove_collator() -> Weight {
		(25_438_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
}
