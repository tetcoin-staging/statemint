
//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-21, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemine-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/statemint
// benchmark
// --chain
// statemine-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_proxy
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./runtime/statemine/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_proxy.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	fn proxy(p: u32, ) -> Weight {
		(41_425_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((27_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(86_865_000 as Weight)
			// Standard Error: 48_000
			.saturating_add((664_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 50_000
			.saturating_add((260_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(53_426_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((781_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 30_000
			.saturating_add((95_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		(63_306_000 as Weight)
			// Standard Error: 53_000
			.saturating_add((600_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn announce(a: u32, p: u32, ) -> Weight {
		(68_365_000 as Weight)
			// Standard Error: 42_000
			.saturating_add((1_019_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 44_000
			.saturating_add((456_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn add_proxy(p: u32, ) -> Weight {
		(58_989_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((206_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_proxy(p: u32, ) -> Weight {
		(57_345_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((190_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_proxies(p: u32, ) -> Weight {
		(52_583_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((203_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn anonymous(p: u32, ) -> Weight {
		(74_677_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((60_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_anonymous(p: u32, ) -> Weight {
		(54_531_000 as Weight)
			// Standard Error: 26_000
			.saturating_add((231_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
