//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use utils::{generate_msm_args, generate_pairing_args, generate_scalar_args};

benchmarks! {
	groth16_verification {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_verification_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_pairing {
		let caller: T::AccountId = whitelisted_caller();
		let (a, b) = generate_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();
	}: _(RawOrigin::Signed(caller), a, b)

	bls12_381_pairing_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (a, b) = generate_pairing_args::<bls12_381::G1AffineOptimized, bls12_381::G2AffineOptimized>();
	}: _(RawOrigin::Signed(caller), a, b)

	bls12_381_msm_g1_10 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>(10);
	}: bls12_381_msm_g1(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>>>(10);
	}: bls12_381_msm_g1_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_1000 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>(1000);
			}: bls12_381_msm_g1(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>>>(1000);
			}: bls12_381_msm_g1_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_10 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>(10);
			}: bls12_381_msm_g2(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>>>(10);
			}: bls12_381_msm_g2_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_1000 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>(1000);
			}: bls12_381_msm_g2(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_msm_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>>>(1000);
			}: bls12_381_msm_g2_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_mul_projective_g1 {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_projective_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<sp_ark_models::short_weierstrass::Affine<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_affine_g1 {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_affine_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_projective_g2 {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_projective_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<sp_ark_models::short_weierstrass::Projective<sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_affine_g2 {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

	bls12_381_mul_affine_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (base, scalar) = generate_scalar_args::<sp_ark_models::short_weierstrass::Affine<sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>>>();
	}: _(RawOrigin::Signed(caller), base, scalar)

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
