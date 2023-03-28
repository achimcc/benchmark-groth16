//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	groth16_verification {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_verification_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_prepare_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let pvk = bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>().unwrap();
	}: _(RawOrigin::Signed(caller), pvk)

	groth16_optimized_prepare_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let pvk = bls12_381::prepare_verifying_key_groth16::<bls12_381::Bls12_381Optimized>().unwrap();
	}: _(RawOrigin::Signed(caller), pvk)

	groth16_prepare_verifying_key {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_optimized_prepare_verifying_key {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_verify_with_prepared_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let pvk = bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>().unwrap();
		let inputs = bls12_381::prepare_inputs_groth16::<ark_bls12_381::Bls12_381>(pvk.clone()).unwrap();
	}: _(RawOrigin::Signed(caller), inputs, pvk)

	groth16_optimized_verify_with_prepared_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let pvk = bls12_381::prepare_verifying_key_groth16::<bls12_381::Bls12_381Optimized>().unwrap();
		let inputs = bls12_381::prepare_inputs_groth16::<bls12_381::Bls12_381Optimized>(pvk.clone()).unwrap();
	}: _(RawOrigin::Signed(caller), inputs, pvk)

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
