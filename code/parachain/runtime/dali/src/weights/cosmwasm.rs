
//! Autogenerated weights for `cosmwasm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-29, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `spm`, CPU: `12th Gen Intel(R) Core(TM) i7-1280P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/jc2araw2q3wsd5zg8708zgqz2cyc0i4b-composable/bin/composable
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
		Weight::from_ref_time(271_893_000 as u64)
			// Standard Error: 250
			.saturating_add(Weight::from_ref_time(59_364 as u64).saturating_mul(n as u64))
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
		Weight::from_ref_time(249_163_000 as u64)
			// Standard Error: 88_981
			.saturating_add(Weight::from_ref_time(26_212_777 as u64).saturating_mul(n as u64))
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
		Weight::from_ref_time(216_840_000 as u64)
			// Standard Error: 84_730
			.saturating_add(Weight::from_ref_time(26_821_314 as u64).saturating_mul(n as u64))
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
		Weight::from_ref_time(437_428_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	fn update_admin() -> Weight {
		Weight::from_ref_time(212_754_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read() -> Weight {
		Weight::from_ref_time(15_092_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read_other_contract() -> Weight {
		Weight::from_ref_time(15_928_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:1)
	fn db_write() -> Weight {
		Weight::from_ref_time(15_420_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn db_scan() -> Weight {
		Weight::from_ref_time(3_563_000 as u64)
	}
	// Storage: unknown [0x] (r:1 w:0)
	fn db_next() -> Weight {
		Weight::from_ref_time(23_644_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:0 w:1)
	fn db_remove() -> Weight {
		Weight::from_ref_time(7_698_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	fn balance() -> Weight {
		Weight::from_ref_time(10_283_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn transfer(n: u32, ) -> Weight {
		Weight::from_ref_time(227_000 as u64)
			// Standard Error: 124_544
			.saturating_add(Weight::from_ref_time(23_850_277 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	fn set_contract_meta() -> Weight {
		Weight::from_ref_time(13_411_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn running_contract_meta() -> Weight {
		Weight::from_ref_time(3_332_000 as u64)
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn contract_meta() -> Weight {
		Weight::from_ref_time(9_975_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn addr_validate() -> Weight {
		Weight::from_ref_time(1_178_000 as u64)
	}
	fn addr_canonicalize() -> Weight {
		Weight::from_ref_time(1_158_000 as u64)
	}
	fn addr_humanize() -> Weight {
		Weight::from_ref_time(235_000 as u64)
	}
	fn secp256k1_recover_pubkey() -> Weight {
		Weight::from_ref_time(33_800_000 as u64)
	}
	fn secp256k1_verify() -> Weight {
		Weight::from_ref_time(33_522_000 as u64)
	}
	fn ed25519_verify() -> Weight {
		Weight::from_ref_time(43_557_000 as u64)
	}
	fn ed25519_batch_verify() -> Weight {
		Weight::from_ref_time(60_427_000 as u64)
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
		Weight::from_ref_time(210_474_000 as u64)
			// Standard Error: 60_778
			.saturating_add(Weight::from_ref_time(23_535_480 as u64).saturating_mul(n as u64))
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
		Weight::from_ref_time(167_013_000 as u64)
			// Standard Error: 37_575
			.saturating_add(Weight::from_ref_time(2_101_405 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn continue_migrate() -> Weight {
		Weight::from_ref_time(195_596_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_query() -> Weight {
		Weight::from_ref_time(170_583_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_reply() -> Weight {
		Weight::from_ref_time(176_336_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:0)
	fn query_contract_info() -> Weight {
		Weight::from_ref_time(14_271_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:0)
	fn query_raw() -> Weight {
		Weight::from_ref_time(25_178_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Const(r: u32, ) -> Weight {
		Weight::from_ref_time(3_214_000 as u64)
			// Standard Error: 2_258
			.saturating_add(Weight::from_ref_time(439_684 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Const(r: u32, ) -> Weight {
		Weight::from_ref_time(2_384_000 as u64)
			// Standard Error: 1_571
			.saturating_add(Weight::from_ref_time(462_678 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Load(r: u32, ) -> Weight {
		Weight::from_ref_time(2_392_000 as u64)
			// Standard Error: 5_244
			.saturating_add(Weight::from_ref_time(1_029_335 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Load(r: u32, ) -> Weight {
		Weight::from_ref_time(2_358_000 as u64)
			// Standard Error: 5_176
			.saturating_add(Weight::from_ref_time(1_050_146 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Store(r: u32, ) -> Weight {
		Weight::from_ref_time(2_364_000 as u64)
			// Standard Error: 10_157
			.saturating_add(Weight::from_ref_time(1_675_136 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Store(r: u32, ) -> Weight {
		Weight::from_ref_time(2_364_000 as u64)
			// Standard Error: 6_115
			.saturating_add(Weight::from_ref_time(1_623_598 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eq(r: u32, ) -> Weight {
		Weight::from_ref_time(2_273_000 as u64)
			// Standard Error: 2_265
			.saturating_add(Weight::from_ref_time(1_135_629 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eqz(r: u32, ) -> Weight {
		Weight::from_ref_time(2_276_000 as u64)
			// Standard Error: 2_428
			.saturating_add(Weight::from_ref_time(894_675 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ne(r: u32, ) -> Weight {
		Weight::from_ref_time(2_272_000 as u64)
			// Standard Error: 4_696
			.saturating_add(Weight::from_ref_time(1_088_147 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LtS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_165_000 as u64)
			// Standard Error: 5_916
			.saturating_add(Weight::from_ref_time(1_070_870 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GtS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_266_000 as u64)
			// Standard Error: 2_173
			.saturating_add(Weight::from_ref_time(1_083_416 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LeS(r: u32, ) -> Weight {
		Weight::from_ref_time(3_660_000 as u64)
			// Standard Error: 11_617
			.saturating_add(Weight::from_ref_time(1_076_426 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GeS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_428_000 as u64)
			// Standard Error: 4_721
			.saturating_add(Weight::from_ref_time(1_101_222 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Clz(r: u32, ) -> Weight {
		Weight::from_ref_time(2_307_000 as u64)
			// Standard Error: 3_952
			.saturating_add(Weight::from_ref_time(854_240 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ctz(r: u32, ) -> Weight {
		Weight::from_ref_time(2_361_000 as u64)
			// Standard Error: 3_817
			.saturating_add(Weight::from_ref_time(892_010 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Popcnt(r: u32, ) -> Weight {
		Weight::from_ref_time(2_322_000 as u64)
			// Standard Error: 7_067
			.saturating_add(Weight::from_ref_time(897_343 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Add(r: u32, ) -> Weight {
		Weight::from_ref_time(2_356_000 as u64)
			// Standard Error: 6_305
			.saturating_add(Weight::from_ref_time(1_083_078 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Sub(r: u32, ) -> Weight {
		Weight::from_ref_time(2_350_000 as u64)
			// Standard Error: 1_912
			.saturating_add(Weight::from_ref_time(1_134_753 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Mul(r: u32, ) -> Weight {
		Weight::from_ref_time(2_388_000 as u64)
			// Standard Error: 6_200
			.saturating_add(Weight::from_ref_time(1_108_763 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_315_000 as u64)
			// Standard Error: 9_309
			.saturating_add(Weight::from_ref_time(1_266_063 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivU(r: u32, ) -> Weight {
		Weight::from_ref_time(2_227_000 as u64)
			// Standard Error: 5_816
			.saturating_add(Weight::from_ref_time(1_260_024 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64RemS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_535_000 as u64)
			// Standard Error: 7_933
			.saturating_add(Weight::from_ref_time(1_306_552 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64And(r: u32, ) -> Weight {
		Weight::from_ref_time(2_333_000 as u64)
			// Standard Error: 4_492
			.saturating_add(Weight::from_ref_time(1_100_351 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Or(r: u32, ) -> Weight {
		Weight::from_ref_time(2_329_000 as u64)
			// Standard Error: 2_269
			.saturating_add(Weight::from_ref_time(1_067_606 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Xor(r: u32, ) -> Weight {
		Weight::from_ref_time(2_267_000 as u64)
			// Standard Error: 9_142
			.saturating_add(Weight::from_ref_time(1_099_155 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Shl(r: u32, ) -> Weight {
		Weight::from_ref_time(2_249_000 as u64)
			// Standard Error: 6_772
			.saturating_add(Weight::from_ref_time(1_180_675 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ShrS(r: u32, ) -> Weight {
		Weight::from_ref_time(2_334_000 as u64)
			// Standard Error: 5_618
			.saturating_add(Weight::from_ref_time(1_092_786 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotl(r: u32, ) -> Weight {
		Weight::from_ref_time(2_204_000 as u64)
			// Standard Error: 2_758
			.saturating_add(Weight::from_ref_time(1_088_154 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotr(r: u32, ) -> Weight {
		Weight::from_ref_time(2_339_000 as u64)
			// Standard Error: 3_430
			.saturating_add(Weight::from_ref_time(1_110_345 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ExtendSI32(r: u32, ) -> Weight {
		Weight::from_ref_time(2_415_000 as u64)
			// Standard Error: 9_167
			.saturating_add(Weight::from_ref_time(948_200 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I32WrapI64(r: u32, ) -> Weight {
		Weight::from_ref_time(2_331_000 as u64)
			// Standard Error: 4_216
			.saturating_add(Weight::from_ref_time(958_400 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Eq(r: u32, ) -> Weight {
		Weight::from_ref_time(3_620_000 as u64)
			// Standard Error: 3_294
			.saturating_add(Weight::from_ref_time(1_034_213 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ne(r: u32, ) -> Weight {
		Weight::from_ref_time(2_205_000 as u64)
			// Standard Error: 2_632
			.saturating_add(Weight::from_ref_time(1_081_909 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Lt(r: u32, ) -> Weight {
		Weight::from_ref_time(3_645_000 as u64)
			// Standard Error: 9_383
			.saturating_add(Weight::from_ref_time(1_108_621 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Gt(r: u32, ) -> Weight {
		Weight::from_ref_time(2_341_000 as u64)
			// Standard Error: 9_146
			.saturating_add(Weight::from_ref_time(1_104_291 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Le(r: u32, ) -> Weight {
		Weight::from_ref_time(2_259_000 as u64)
			// Standard Error: 7_593
			.saturating_add(Weight::from_ref_time(1_103_513 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ge(r: u32, ) -> Weight {
		Weight::from_ref_time(2_337_000 as u64)
			// Standard Error: 5_273
			.saturating_add(Weight::from_ref_time(1_097_993 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Abs(r: u32, ) -> Weight {
		Weight::from_ref_time(2_364_000 as u64)
			// Standard Error: 3_217
			.saturating_add(Weight::from_ref_time(1_035_855 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Neg(r: u32, ) -> Weight {
		Weight::from_ref_time(2_392_000 as u64)
			// Standard Error: 1_905
			.saturating_add(Weight::from_ref_time(835_275 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ceil(r: u32, ) -> Weight {
		Weight::from_ref_time(2_350_000 as u64)
			// Standard Error: 4_657
			.saturating_add(Weight::from_ref_time(1_061_170 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Floor(r: u32, ) -> Weight {
		Weight::from_ref_time(2_246_000 as u64)
			// Standard Error: 5_270
			.saturating_add(Weight::from_ref_time(1_057_734 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Trunc(r: u32, ) -> Weight {
		Weight::from_ref_time(2_314_000 as u64)
			// Standard Error: 3_518
			.saturating_add(Weight::from_ref_time(1_016_242 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Nearest(r: u32, ) -> Weight {
		Weight::from_ref_time(2_354_000 as u64)
			// Standard Error: 2_928
			.saturating_add(Weight::from_ref_time(1_358_859 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sqrt(r: u32, ) -> Weight {
		Weight::from_ref_time(2_341_000 as u64)
			// Standard Error: 2_606
			.saturating_add(Weight::from_ref_time(1_025_581 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Add(r: u32, ) -> Weight {
		Weight::from_ref_time(2_360_000 as u64)
			// Standard Error: 2_313
			.saturating_add(Weight::from_ref_time(1_092_755 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sub(r: u32, ) -> Weight {
		Weight::from_ref_time(2_236_000 as u64)
			// Standard Error: 2_101
			.saturating_add(Weight::from_ref_time(1_130_604 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Mul(r: u32, ) -> Weight {
		Weight::from_ref_time(2_326_000 as u64)
			// Standard Error: 3_430
			.saturating_add(Weight::from_ref_time(1_088_270 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Div(r: u32, ) -> Weight {
		Weight::from_ref_time(2_263_000 as u64)
			// Standard Error: 6_367
			.saturating_add(Weight::from_ref_time(1_117_036 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Min(r: u32, ) -> Weight {
		Weight::from_ref_time(2_293_000 as u64)
			// Standard Error: 7_038
			.saturating_add(Weight::from_ref_time(1_308_341 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Max(r: u32, ) -> Weight {
		Weight::from_ref_time(2_289_000 as u64)
			// Standard Error: 9_014
			.saturating_add(Weight::from_ref_time(1_311_468 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Copysign(r: u32, ) -> Weight {
		Weight::from_ref_time(2_208_000 as u64)
			// Standard Error: 3_553
			.saturating_add(Weight::from_ref_time(1_127_629 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Select(r: u32, ) -> Weight {
		Weight::from_ref_time(2_289_000 as u64)
			// Standard Error: 10_175
			.saturating_add(Weight::from_ref_time(1_312_505 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_If(r: u32, ) -> Weight {
		Weight::from_ref_time(2_282_000 as u64)
			// Standard Error: 2_171
			.saturating_add(Weight::from_ref_time(635_985 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Else(r: u32, ) -> Weight {
		Weight::from_ref_time(2_558_000 as u64)
			// Standard Error: 3_598
			.saturating_add(Weight::from_ref_time(958_038 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetLocal(r: u32, ) -> Weight {
		Weight::from_ref_time(2_965_000 as u64)
			// Standard Error: 2_430
			.saturating_add(Weight::from_ref_time(460_828 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetLocal(r: u32, ) -> Weight {
		Weight::from_ref_time(2_255_000 as u64)
			// Standard Error: 3_132
			.saturating_add(Weight::from_ref_time(812_317 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_TeeLocal(r: u32, ) -> Weight {
		Weight::from_ref_time(2_312_000 as u64)
			// Standard Error: 1_197
			.saturating_add(Weight::from_ref_time(11_080 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetGlobal(r: u32, ) -> Weight {
		Weight::from_ref_time(2_430_000 as u64)
			// Standard Error: 342
			.saturating_add(Weight::from_ref_time(6_784 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetGlobal(r: u32, ) -> Weight {
		Weight::from_ref_time(2_501_000 as u64)
			// Standard Error: 262
			.saturating_add(Weight::from_ref_time(4_817 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CurrentMemory(r: u32, ) -> Weight {
		Weight::from_ref_time(2_460_000 as u64)
			// Standard Error: 6_538
			.saturating_add(Weight::from_ref_time(944_305 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 3]`.
	fn instruction_GrowMemory(r: u32, ) -> Weight {
		Weight::from_ref_time(2_417_000 as u64)
			// Standard Error: 10_073_873
			.saturating_add(Weight::from_ref_time(1_357_098_312 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Br(r: u32, ) -> Weight {
		Weight::from_ref_time(2_396_000 as u64)
			// Standard Error: 3_254
			.saturating_add(Weight::from_ref_time(529_449 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrIf(r: u32, ) -> Weight {
		Weight::from_ref_time(2_552_000 as u64)
			// Standard Error: 4_786
			.saturating_add(Weight::from_ref_time(808_752 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrTable(r: u32, ) -> Weight {
		Weight::from_ref_time(2_431_000 as u64)
			// Standard Error: 7_590
			.saturating_add(Weight::from_ref_time(1_166_730 as u64).saturating_mul(r as u64))
	}
	/// The range of component `s` is `[1, 50]`.
	fn instruction_BrTable_per_elem(s: u32, ) -> Weight {
		Weight::from_ref_time(3_632_000 as u64)
			// Standard Error: 1_461
			.saturating_add(Weight::from_ref_time(13_088 as u64).saturating_mul(s as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Call(r: u32, ) -> Weight {
		Weight::from_ref_time(2_486_000 as u64)
			// Standard Error: 13_595
			.saturating_add(Weight::from_ref_time(6_753_520 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CallIndirect(r: u32, ) -> Weight {
		Weight::from_ref_time(2_558_000 as u64)
			// Standard Error: 11_652
			.saturating_add(Weight::from_ref_time(7_972_635 as u64).saturating_mul(r as u64))
	}
}
