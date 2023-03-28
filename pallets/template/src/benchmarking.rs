//! Benchmarking setup for pallet-template

use super::*;

use crate::bls12_381::BlsFrOptimized;
#[allow(unused)]
use crate::Pallet as Template;
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::{Fp, MontBackend};
use ark_groth16::Groth16;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	groth16_verification {
		let caller: T::AccountId = whitelisted_caller();

		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

		let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();
		let c = utils::serialize_argument(c);

		let proof = <Groth16<Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
			bls12_381::PROOF_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let proof = utils::serialize_argument(proof);
	}: _(RawOrigin::Signed(caller), vk, c, proof)

	groth16_verification_optimized {
		let caller: T::AccountId = whitelisted_caller();

		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

		let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();

		let c = utils::serialize_argument(c);

		let proof = <Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
			bls12_381::PROOF_SERIALIZED,
			Compress::No,
			Validate::No,
		)
		.unwrap();
		let proof = utils::serialize_argument(proof);
	}: _(RawOrigin::Signed(caller), vk, c, proof)

	groth16_prepare_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);
		let pvk = bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>(vk).unwrap();

		let c: Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4> = Fp::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();
		let c = utils::serialize_argument(c);
	}: _(RawOrigin::Signed(caller), pvk, c)

	groth16_optimized_prepare_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

		let pvk = bls12_381::prepare_verifying_key_groth16::<bls12_381::Bls12_381Optimized>(vk).unwrap();

		let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();
		let c = utils::serialize_argument(c);
	}: _(RawOrigin::Signed(caller), pvk, c)

	groth16_prepare_verifying_key {
		let caller: T::AccountId = whitelisted_caller();

		let vk = <Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

	}: _(RawOrigin::Signed(caller), vk)

	groth16_optimized_prepare_verifying_key {
		let caller: T::AccountId = whitelisted_caller();

		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

	}: _(RawOrigin::Signed(caller), vk)

	groth16_verify_with_prepared_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);

		let pvk = bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>(vk).unwrap();

		let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();
		let c = utils::serialize_argument(c);

		let inputs = bls12_381::prepare_inputs_groth16::<ark_bls12_381::Bls12_381>(pvk.clone(), c).unwrap();

		let proof = <Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFrOptimized>>::Proof::deserialize_with_mode(
			bls12_381::PROOF_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let proof = utils::serialize_argument(proof);
	}: _(RawOrigin::Signed(caller), inputs, pvk, proof)

	groth16_optimized_verify_with_prepared_inputs {
		let caller: T::AccountId = whitelisted_caller();
		let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
			bls12_381::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = utils::serialize_argument(vk);
		let pvk = bls12_381::prepare_verifying_key_groth16::<bls12_381::Bls12_381Optimized>(vk).unwrap();
		let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(bls12_381::C_SERIALIZED, Compress::Yes, Validate::No).unwrap();
		let c = utils::serialize_argument(c);
		let inputs = bls12_381::prepare_inputs_groth16::<bls12_381::Bls12_381Optimized>(pvk.clone(), c).unwrap();
		let proof = <Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFrOptimized>>::Proof::deserialize_with_mode(
			bls12_381::PROOF_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let proof = utils::serialize_argument(proof);
	}: _(RawOrigin::Signed(caller), inputs, pvk, proof)

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
