#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "1024"]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod bls12_381;
pub mod utils;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		bls12_381,
		bls12_381::{Bls12_381Optimized, BlsFrOptimized},
		utils,
	};
	use ark_bls12_381::Fr as BlsFr;
	use ark_ff::Fp;
	use ark_groth16::Groth16;
	use ark_serialize::{CanonicalDeserialize, Compress, Validate};
	use ark_snark::SNARK;
	use ark_std::{io::Cursor, vec, vec::Vec};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Successfull groth16 verification event
		VerificationSuccess { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Verification of groth16 proof failed
		VerificationFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification(
			origin: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let cursor = Cursor::new(&vk);
			let vk = <Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let vk = utils::serialize_argument(vk);

			let cursor = Cursor::new(&c);
			let c: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4> =
				Fp::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let c = utils::serialize_argument(c);

			let cursor = Cursor::new(&proof);
			let proof =
				<Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
					cursor,
					Compress::No,
					Validate::No,
				)
				.unwrap();
			let proof = utils::serialize_argument(proof);

			let result = crate::bls12_381::do_verify_groth16(vk, c, proof);

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification_optimized(
			origin: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let cursor = Cursor::new(&vk);
			let vk = <Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let vk = utils::serialize_argument(vk);

			let cursor = Cursor::new(&c);
			let c: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4> =
				Fp::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let c = utils::serialize_argument(c);

			let cursor = Cursor::new(&proof);
			let proof =
				<Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
					cursor,
					Compress::No,
					Validate::No,
				)
				.unwrap();
			let proof = utils::serialize_argument(proof);

			let result = crate::bls12_381::do_verify_groth16_optimized(vk, c, proof);

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_prepare_inputs(
			_origin: OriginFor<T>,
			pvk: Vec<u8>,
			c: Vec<u8>,
		) -> DispatchResult {
			let _inputs =
				crate::bls12_381::prepare_inputs_groth16::<ark_bls12_381::Bls12_381>(pvk, c)
					.unwrap();
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_prepare_inputs(
			_origin: OriginFor<T>,
			pvk: Vec<u8>,
			c: Vec<u8>,
		) -> DispatchResult {
			let _inputs =
				crate::bls12_381::prepare_inputs_groth16::<bls12_381::Bls12_381Optimized>(pvk, c)
					.unwrap();
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verify_with_prepared_inputs(
			_origin: OriginFor<T>,
			inputs: Vec<u8>,
			pvk: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			crate::bls12_381::verify_with_prepared_inputs_groth16::<ark_bls12_381::Bls12_381>(
				inputs, pvk, proof,
			)
			.unwrap();
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_verify_with_prepared_inputs(
			_origin: OriginFor<T>,
			inputs: Vec<u8>,
			pvk: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			crate::bls12_381::verify_with_prepared_inputs_groth16::<bls12_381::Bls12_381Optimized>(
				inputs, pvk, proof,
			)
			.unwrap();
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_prepare_verifying_key(_origin: OriginFor<T>, vk: Vec<u8>) -> DispatchResult {
			let cursor = Cursor::new(&vk);
			let vk = <Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let vk = utils::serialize_argument(vk);

			let _pvk =
				crate::bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>(vk)
					.unwrap();
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_prepare_verifying_key(
			_origin: OriginFor<T>,
			vk: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(&vk);
			let vk = <Groth16<bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let vk = utils::serialize_argument(vk);

			let _pvk = crate::bls12_381::prepare_verifying_key_groth16::<
				bls12_381::Bls12_381Optimized,
			>(vk)
			.unwrap();
			Ok(())
		}
	}
}
