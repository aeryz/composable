
//! Autogenerated weights for `cosmwasm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-07, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `spm`, CPU: `12th Gen Intel(R) Core(TM) i7-1280P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/dcf9bqq6c6v1iyyznr23j5xm4jfvrh8b-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cosmwasm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cosmwasm::WeightInfo for WeightInfo<T> {
	// Storage: Cosmwasm CodeHashToId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Cosmwasm CurrentCodeId (r:1 w:1)
	// Storage: Cosmwasm PristineCode (r:0 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:0 w:1)
	// Storage: Cosmwasm CodeIdToInfo (r:0 w:1)
	/// The range of component `n` is `[1, 514288]`.
	fn upload(n: u32, ) -> Weight {
		// Minimum execution time: 271_574 nanoseconds.
		Weight::from_ref_time(162_648_596 as u64)
			// Standard Error: 403
			.saturating_add(Weight::from_ref_time(53_527 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CurrentNonce (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn instantiate(n: u32, ) -> Weight {
		// Minimum execution time: 198_140 nanoseconds.
		Weight::from_ref_time(287_214_993 as u64)
			// Standard Error: 284_906
			.saturating_add(Weight::from_ref_time(21_049_241 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn execute(n: u32, ) -> Weight {
		// Minimum execution time: 171_902 nanoseconds.
		Weight::from_ref_time(228_110_923 as u64)
			// Standard Error: 248_251
			.saturating_add(Weight::from_ref_time(22_437_687 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm CodeIdToInfo (r:2 w:2)
	// Storage: Cosmwasm InstrumentedCode (r:2 w:1)
	// Storage: Cosmwasm PristineCode (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Cosmwasm CodeHashToId (r:0 w:1)
	fn migrate() -> Weight {
		// Minimum execution time: 384_162 nanoseconds.
		Weight::from_ref_time(419_025_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	fn update_admin() -> Weight {
		// Minimum execution time: 163_850 nanoseconds.
		Weight::from_ref_time(191_253_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read() -> Weight {
		// Minimum execution time: 11_742 nanoseconds.
		Weight::from_ref_time(19_398_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read_other_contract() -> Weight {
		// Minimum execution time: 12_303 nanoseconds.
		Weight::from_ref_time(13_764_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:1)
	fn db_write() -> Weight {
		// Minimum execution time: 12_148 nanoseconds.
		Weight::from_ref_time(13_166_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn db_scan() -> Weight {
		// Minimum execution time: 3_613 nanoseconds.
		Weight::from_ref_time(3_852_000 as u64)
	}
	// Storage: unknown [0x] (r:1 w:0)
	fn db_next() -> Weight {
		// Minimum execution time: 25_046 nanoseconds.
		Weight::from_ref_time(26_100_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:0 w:1)
	fn db_remove() -> Weight {
		// Minimum execution time: 7_320 nanoseconds.
		Weight::from_ref_time(7_545_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	fn balance() -> Weight {
		// Minimum execution time: 4_605 nanoseconds.
		Weight::from_ref_time(4_792_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn transfer(n: u32, ) -> Weight {
		// Minimum execution time: 185 nanoseconds.
		Weight::from_ref_time(28_829_703 as u64)
			// Standard Error: 251_813
			.saturating_add(Weight::from_ref_time(20_255_029 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	fn set_contract_meta() -> Weight {
		// Minimum execution time: 10_897 nanoseconds.
		Weight::from_ref_time(12_169_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn running_contract_meta() -> Weight {
		// Minimum execution time: 2_782 nanoseconds.
		Weight::from_ref_time(2_958_000 as u64)
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn contract_meta() -> Weight {
		// Minimum execution time: 9_129 nanoseconds.
		Weight::from_ref_time(10_074_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn addr_validate() -> Weight {
		// Minimum execution time: 2_327 nanoseconds.
		Weight::from_ref_time(2_394_000 as u64)
	}
	fn addr_canonicalize() -> Weight {
		// Minimum execution time: 2_298 nanoseconds.
		Weight::from_ref_time(2_405_000 as u64)
	}
	fn addr_humanize() -> Weight {
		// Minimum execution time: 182 nanoseconds.
		Weight::from_ref_time(193_000 as u64)
	}
	fn secp256k1_recover_pubkey() -> Weight {
		// Minimum execution time: 33_261 nanoseconds.
		Weight::from_ref_time(33_478_000 as u64)
	}
	fn secp256k1_verify() -> Weight {
		// Minimum execution time: 33_284 nanoseconds.
		Weight::from_ref_time(33_515_000 as u64)
	}
	fn ed25519_verify() -> Weight {
		// Minimum execution time: 43_115 nanoseconds.
		Weight::from_ref_time(43_412_000 as u64)
	}
	fn ed25519_batch_verify() -> Weight {
		// Minimum execution time: 53_731 nanoseconds.
		Weight::from_ref_time(57_591_000 as u64)
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CurrentNonce (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn continue_instantiate(n: u32, ) -> Weight {
		// Minimum execution time: 158_953 nanoseconds.
		Weight::from_ref_time(208_075_967 as u64)
			// Standard Error: 211_678
			.saturating_add(Weight::from_ref_time(21_239_815 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	/// The range of component `n` is `[0, 23]`.
	fn continue_execute(n: u32, ) -> Weight {
		// Minimum execution time: 135_624 nanoseconds.
		Weight::from_ref_time(172_822_115 as u64)
			// Standard Error: 100_901
			.saturating_add(Weight::from_ref_time(1_252_051 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn continue_migrate() -> Weight {
		// Minimum execution time: 140_979 nanoseconds.
		Weight::from_ref_time(150_121_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_query() -> Weight {
		// Minimum execution time: 143_627 nanoseconds.
		Weight::from_ref_time(171_952_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_reply() -> Weight {
		// Minimum execution time: 135_144 nanoseconds.
		Weight::from_ref_time(142_901_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:0)
	fn query_contract_info() -> Weight {
		// Minimum execution time: 14_984 nanoseconds.
		Weight::from_ref_time(17_443_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:0)
	fn query_code_info() -> Weight {
		// Minimum execution time: 11_706 nanoseconds.
		Weight::from_ref_time(12_557_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:0)
	fn query_raw() -> Weight {
		// Minimum execution time: 21_484 nanoseconds.
		Weight::from_ref_time(24_350_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Const(r: u32, ) -> Weight {
		// Minimum execution time: 2_321 nanoseconds.
		Weight::from_ref_time(3_190_413 as u64)
			// Standard Error: 4_032
			.saturating_add(Weight::from_ref_time(392_054 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Const(r: u32, ) -> Weight {
		// Minimum execution time: 2_005 nanoseconds.
		Weight::from_ref_time(3_364_583 as u64)
			// Standard Error: 4_263
			.saturating_add(Weight::from_ref_time(384_523 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Load(r: u32, ) -> Weight {
		// Minimum execution time: 1_963 nanoseconds.
		Weight::from_ref_time(3_990_141 as u64)
			// Standard Error: 9_274
			.saturating_add(Weight::from_ref_time(913_236 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Load(r: u32, ) -> Weight {
		// Minimum execution time: 1_951 nanoseconds.
		Weight::from_ref_time(3_247_696 as u64)
			// Standard Error: 17_144
			.saturating_add(Weight::from_ref_time(912_574 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Store(r: u32, ) -> Weight {
		// Minimum execution time: 1_745 nanoseconds.
		Weight::from_ref_time(4_346_438 as u64)
			// Standard Error: 15_231
			.saturating_add(Weight::from_ref_time(1_544_619 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Store(r: u32, ) -> Weight {
		// Minimum execution time: 2_125 nanoseconds.
		Weight::from_ref_time(5_616_947 as u64)
			// Standard Error: 19_891
			.saturating_add(Weight::from_ref_time(1_389_021 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eq(r: u32, ) -> Weight {
		// Minimum execution time: 1_921 nanoseconds.
		Weight::from_ref_time(3_593_636 as u64)
			// Standard Error: 8_139
			.saturating_add(Weight::from_ref_time(1_023_245 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eqz(r: u32, ) -> Weight {
		// Minimum execution time: 2_194 nanoseconds.
		Weight::from_ref_time(2_445_827 as u64)
			// Standard Error: 15_741
			.saturating_add(Weight::from_ref_time(888_240 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ne(r: u32, ) -> Weight {
		// Minimum execution time: 2_305 nanoseconds.
		Weight::from_ref_time(4_198_901 as u64)
			// Standard Error: 5_223
			.saturating_add(Weight::from_ref_time(1_073_739 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LtS(r: u32, ) -> Weight {
		// Minimum execution time: 1_901 nanoseconds.
		Weight::from_ref_time(3_691_782 as u64)
			// Standard Error: 4_899
			.saturating_add(Weight::from_ref_time(1_028_688 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GtS(r: u32, ) -> Weight {
		// Minimum execution time: 1_989 nanoseconds.
		Weight::from_ref_time(3_487_985 as u64)
			// Standard Error: 4_183
			.saturating_add(Weight::from_ref_time(1_030_899 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LeS(r: u32, ) -> Weight {
		// Minimum execution time: 2_270 nanoseconds.
		Weight::from_ref_time(3_998_697 as u64)
			// Standard Error: 15_208
			.saturating_add(Weight::from_ref_time(1_036_222 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GeS(r: u32, ) -> Weight {
		// Minimum execution time: 2_006 nanoseconds.
		Weight::from_ref_time(3_941_043 as u64)
			// Standard Error: 8_039
			.saturating_add(Weight::from_ref_time(1_054_837 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Clz(r: u32, ) -> Weight {
		// Minimum execution time: 1_984 nanoseconds.
		Weight::from_ref_time(3_480_999 as u64)
			// Standard Error: 11_419
			.saturating_add(Weight::from_ref_time(764_760 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ctz(r: u32, ) -> Weight {
		// Minimum execution time: 1_992 nanoseconds.
		Weight::from_ref_time(3_434_132 as u64)
			// Standard Error: 6_483
			.saturating_add(Weight::from_ref_time(766_942 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Popcnt(r: u32, ) -> Weight {
		// Minimum execution time: 2_049 nanoseconds.
		Weight::from_ref_time(3_755_275 as u64)
			// Standard Error: 11_730
			.saturating_add(Weight::from_ref_time(795_977 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Add(r: u32, ) -> Weight {
		// Minimum execution time: 2_100 nanoseconds.
		Weight::from_ref_time(4_266_032 as u64)
			// Standard Error: 8_996
			.saturating_add(Weight::from_ref_time(1_039_996 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Sub(r: u32, ) -> Weight {
		// Minimum execution time: 2_027 nanoseconds.
		Weight::from_ref_time(3_425_301 as u64)
			// Standard Error: 5_307
			.saturating_add(Weight::from_ref_time(1_028_835 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Mul(r: u32, ) -> Weight {
		// Minimum execution time: 2_226 nanoseconds.
		Weight::from_ref_time(3_214_769 as u64)
			// Standard Error: 10_394
			.saturating_add(Weight::from_ref_time(1_090_904 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivS(r: u32, ) -> Weight {
		// Minimum execution time: 2_095 nanoseconds.
		Weight::from_ref_time(3_265_826 as u64)
			// Standard Error: 5_427
			.saturating_add(Weight::from_ref_time(1_209_918 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivU(r: u32, ) -> Weight {
		// Minimum execution time: 2_122 nanoseconds.
		Weight::from_ref_time(4_517_028 as u64)
			// Standard Error: 18_854
			.saturating_add(Weight::from_ref_time(1_097_518 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64RemS(r: u32, ) -> Weight {
		// Minimum execution time: 2_046 nanoseconds.
		Weight::from_ref_time(3_949_914 as u64)
			// Standard Error: 8_394
			.saturating_add(Weight::from_ref_time(1_174_367 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64And(r: u32, ) -> Weight {
		// Minimum execution time: 2_125 nanoseconds.
		Weight::from_ref_time(3_652_492 as u64)
			// Standard Error: 4_059
			.saturating_add(Weight::from_ref_time(1_034_281 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Or(r: u32, ) -> Weight {
		// Minimum execution time: 2_038 nanoseconds.
		Weight::from_ref_time(3_693_491 as u64)
			// Standard Error: 8_085
			.saturating_add(Weight::from_ref_time(1_041_688 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Xor(r: u32, ) -> Weight {
		// Minimum execution time: 1_965 nanoseconds.
		Weight::from_ref_time(4_293_168 as u64)
			// Standard Error: 5_329
			.saturating_add(Weight::from_ref_time(986_019 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Shl(r: u32, ) -> Weight {
		// Minimum execution time: 2_046 nanoseconds.
		Weight::from_ref_time(3_680_234 as u64)
			// Standard Error: 4_714
			.saturating_add(Weight::from_ref_time(1_050_559 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ShrS(r: u32, ) -> Weight {
		// Minimum execution time: 2_001 nanoseconds.
		Weight::from_ref_time(3_519_300 as u64)
			// Standard Error: 7_620
			.saturating_add(Weight::from_ref_time(1_074_830 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotl(r: u32, ) -> Weight {
		// Minimum execution time: 2_191 nanoseconds.
		Weight::from_ref_time(4_082_972 as u64)
			// Standard Error: 8_143
			.saturating_add(Weight::from_ref_time(1_038_234 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotr(r: u32, ) -> Weight {
		// Minimum execution time: 1_974 nanoseconds.
		Weight::from_ref_time(3_898_684 as u64)
			// Standard Error: 7_693
			.saturating_add(Weight::from_ref_time(1_072_370 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ExtendSI32(r: u32, ) -> Weight {
		// Minimum execution time: 2_189 nanoseconds.
		Weight::from_ref_time(3_322_333 as u64)
			// Standard Error: 11_269
			.saturating_add(Weight::from_ref_time(835_851 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I32WrapI64(r: u32, ) -> Weight {
		// Minimum execution time: 2_308 nanoseconds.
		Weight::from_ref_time(3_466_641 as u64)
			// Standard Error: 3_959
			.saturating_add(Weight::from_ref_time(864_161 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Eq(r: u32, ) -> Weight {
		// Minimum execution time: 2_132 nanoseconds.
		Weight::from_ref_time(3_847_575 as u64)
			// Standard Error: 13_130
			.saturating_add(Weight::from_ref_time(1_069_883 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ne(r: u32, ) -> Weight {
		// Minimum execution time: 2_029 nanoseconds.
		Weight::from_ref_time(3_044_086 as u64)
			// Standard Error: 18_080
			.saturating_add(Weight::from_ref_time(1_069_844 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Lt(r: u32, ) -> Weight {
		// Minimum execution time: 1_946 nanoseconds.
		Weight::from_ref_time(4_062_958 as u64)
			// Standard Error: 5_113
			.saturating_add(Weight::from_ref_time(997_861 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Gt(r: u32, ) -> Weight {
		// Minimum execution time: 1_952 nanoseconds.
		Weight::from_ref_time(3_658_420 as u64)
			// Standard Error: 4_960
			.saturating_add(Weight::from_ref_time(1_074_843 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Le(r: u32, ) -> Weight {
		// Minimum execution time: 2_013 nanoseconds.
		Weight::from_ref_time(3_927_814 as u64)
			// Standard Error: 15_663
			.saturating_add(Weight::from_ref_time(974_573 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ge(r: u32, ) -> Weight {
		// Minimum execution time: 2_056 nanoseconds.
		Weight::from_ref_time(3_857_147 as u64)
			// Standard Error: 3_951
			.saturating_add(Weight::from_ref_time(1_009_137 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Abs(r: u32, ) -> Weight {
		// Minimum execution time: 2_058 nanoseconds.
		Weight::from_ref_time(2_990_744 as u64)
			// Standard Error: 9_478
			.saturating_add(Weight::from_ref_time(1_026_933 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Neg(r: u32, ) -> Weight {
		// Minimum execution time: 2_101 nanoseconds.
		Weight::from_ref_time(3_723_297 as u64)
			// Standard Error: 12_152
			.saturating_add(Weight::from_ref_time(755_956 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ceil(r: u32, ) -> Weight {
		// Minimum execution time: 1_998 nanoseconds.
		Weight::from_ref_time(3_943_557 as u64)
			// Standard Error: 12_923
			.saturating_add(Weight::from_ref_time(914_491 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Floor(r: u32, ) -> Weight {
		// Minimum execution time: 2_026 nanoseconds.
		Weight::from_ref_time(4_416_107 as u64)
			// Standard Error: 11_367
			.saturating_add(Weight::from_ref_time(914_349 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Trunc(r: u32, ) -> Weight {
		// Minimum execution time: 1_951 nanoseconds.
		Weight::from_ref_time(3_440_765 as u64)
			// Standard Error: 5_055
			.saturating_add(Weight::from_ref_time(1_023_190 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Nearest(r: u32, ) -> Weight {
		// Minimum execution time: 1_969 nanoseconds.
		Weight::from_ref_time(2_841_517 as u64)
			// Standard Error: 16_303
			.saturating_add(Weight::from_ref_time(1_325_681 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sqrt(r: u32, ) -> Weight {
		// Minimum execution time: 1_993 nanoseconds.
		Weight::from_ref_time(3_780_178 as u64)
			// Standard Error: 8_926
			.saturating_add(Weight::from_ref_time(914_044 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Add(r: u32, ) -> Weight {
		// Minimum execution time: 2_232 nanoseconds.
		Weight::from_ref_time(4_237_355 as u64)
			// Standard Error: 8_105
			.saturating_add(Weight::from_ref_time(1_066_731 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sub(r: u32, ) -> Weight {
		// Minimum execution time: 2_016 nanoseconds.
		Weight::from_ref_time(4_239_468 as u64)
			// Standard Error: 16_601
			.saturating_add(Weight::from_ref_time(1_026_053 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Mul(r: u32, ) -> Weight {
		// Minimum execution time: 2_051 nanoseconds.
		Weight::from_ref_time(3_535_345 as u64)
			// Standard Error: 13_934
			.saturating_add(Weight::from_ref_time(1_075_490 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Div(r: u32, ) -> Weight {
		// Minimum execution time: 1_992 nanoseconds.
		Weight::from_ref_time(4_151_280 as u64)
			// Standard Error: 6_410
			.saturating_add(Weight::from_ref_time(1_028_654 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Min(r: u32, ) -> Weight {
		// Minimum execution time: 2_027 nanoseconds.
		Weight::from_ref_time(4_669_244 as u64)
			// Standard Error: 12_478
			.saturating_add(Weight::from_ref_time(1_216_073 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Max(r: u32, ) -> Weight {
		// Minimum execution time: 2_202 nanoseconds.
		Weight::from_ref_time(3_475_756 as u64)
			// Standard Error: 6_096
			.saturating_add(Weight::from_ref_time(1_290_564 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Copysign(r: u32, ) -> Weight {
		// Minimum execution time: 2_121 nanoseconds.
		Weight::from_ref_time(3_859_566 as u64)
			// Standard Error: 7_606
			.saturating_add(Weight::from_ref_time(1_017_120 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Select(r: u32, ) -> Weight {
		// Minimum execution time: 2_093 nanoseconds.
		Weight::from_ref_time(4_103_274 as u64)
			// Standard Error: 6_160
			.saturating_add(Weight::from_ref_time(1_187_108 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_If(r: u32, ) -> Weight {
		// Minimum execution time: 2_278 nanoseconds.
		Weight::from_ref_time(2_985_342 as u64)
			// Standard Error: 10_693
			.saturating_add(Weight::from_ref_time(589_129 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Else(r: u32, ) -> Weight {
		// Minimum execution time: 2_106 nanoseconds.
		Weight::from_ref_time(3_970_960 as u64)
			// Standard Error: 10_872
			.saturating_add(Weight::from_ref_time(953_200 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetLocal(r: u32, ) -> Weight {
		// Minimum execution time: 2_104 nanoseconds.
		Weight::from_ref_time(3_390_479 as u64)
			// Standard Error: 7_805
			.saturating_add(Weight::from_ref_time(421_734 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetLocal(r: u32, ) -> Weight {
		// Minimum execution time: 2_263 nanoseconds.
		Weight::from_ref_time(3_022_469 as u64)
			// Standard Error: 10_278
			.saturating_add(Weight::from_ref_time(489_427 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_TeeLocal(_r: u32, ) -> Weight {
		// Minimum execution time: 2_224 nanoseconds.
		Weight::from_ref_time(2_639_100 as u64)
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetGlobal(r: u32, ) -> Weight {
		// Minimum execution time: 2_232 nanoseconds.
		Weight::from_ref_time(2_428_832 as u64)
			// Standard Error: 1_442
			.saturating_add(Weight::from_ref_time(5_542 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetGlobal(r: u32, ) -> Weight {
		// Minimum execution time: 2_122 nanoseconds.
		Weight::from_ref_time(2_493_749 as u64)
			// Standard Error: 1_405
			.saturating_add(Weight::from_ref_time(1_494 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CurrentMemory(r: u32, ) -> Weight {
		// Minimum execution time: 2_289 nanoseconds.
		Weight::from_ref_time(3_429_903 as u64)
			// Standard Error: 7_769
			.saturating_add(Weight::from_ref_time(801_084 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 3]`.
	fn instruction_GrowMemory(r: u32, ) -> Weight {
		// Minimum execution time: 2_236 nanoseconds.
		Weight::from_ref_time(2_338_000 as u64)
			// Standard Error: 7_980_554
			.saturating_add(Weight::from_ref_time(1_314_142_270 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Br(r: u32, ) -> Weight {
		// Minimum execution time: 2_215 nanoseconds.
		Weight::from_ref_time(2_778_399 as u64)
			// Standard Error: 6_943
			.saturating_add(Weight::from_ref_time(480_979 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrIf(r: u32, ) -> Weight {
		// Minimum execution time: 3_126 nanoseconds.
		Weight::from_ref_time(3_171_100 as u64)
			// Standard Error: 10_714
			.saturating_add(Weight::from_ref_time(725_037 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrTable(r: u32, ) -> Weight {
		// Minimum execution time: 1_985 nanoseconds.
		Weight::from_ref_time(3_210_530 as u64)
			// Standard Error: 20_324
			.saturating_add(Weight::from_ref_time(1_128_892 as u64).saturating_mul(r as u64))
	}
	/// The range of component `s` is `[1, 50]`.
	fn instruction_BrTable_per_elem(s: u32, ) -> Weight {
		// Minimum execution time: 3_299 nanoseconds.
		Weight::from_ref_time(3_793_259 as u64)
			// Standard Error: 2_035
			.saturating_add(Weight::from_ref_time(7_301 as u64).saturating_mul(s as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Call(r: u32, ) -> Weight {
		// Minimum execution time: 2_355 nanoseconds.
		Weight::from_ref_time(11_818_591 as u64)
			// Standard Error: 37_068
			.saturating_add(Weight::from_ref_time(6_078_169 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CallIndirect(r: u32, ) -> Weight {
		// Minimum execution time: 2_222 nanoseconds.
		Weight::from_ref_time(11_645_698 as u64)
			// Standard Error: 25_976
			.saturating_add(Weight::from_ref_time(7_209_483 as u64).saturating_mul(r as u64))
	}
}
