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
	use crate::bls12_381;
	use ark_serialize::{CanonicalDeserialize, Compress, Validate};
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
		pub fn groth16_verification(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let result = crate::bls12_381::do_verify_groth16();

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification_optimized(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let result = crate::bls12_381::do_verify_groth16_optimized();

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing(_origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>) -> DispatchResult {
			let cursor = Cursor::new(a);
			let a =
				ark_bls12_381::G1Affine::deserialize_with_mode(cursor, Compress::No, Validate::No)
					.unwrap();
			let cursor = Cursor::new(b);
			let b =
				ark_bls12_381::G2Affine::deserialize_with_mode(cursor, Compress::No, Validate::No)
					.unwrap();
			let _ = crate::bls12_381::do_pairing(a, b);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing_optimized(
			_origin: OriginFor<T>,
			a: Vec<u8>,
			b: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(a);
			let a = bls12_381::G1AffineOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(b);
			let b = bls12_381::G2AffineOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let _ = crate::bls12_381::do_pairing_optimized(a, b);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					ark_bls12_381::G1Affine::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::g1::Config as ark_ec::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1_optimized(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					bls12_381::G1Affine_Host::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381> as sp_ark_models::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::Bls12_381 as ark_ec::pairing::Pairing>::G2Affine::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::g2::Config as ark_ec::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2_optimized(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					bls12_381::G2AffineOptimized::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381> as sp_ark_models::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = ark_bls12_381::G1Projective::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_projective_g1(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = bls12_381::G1ProjectiveOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_projective_g1_optimized(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base =
				ark_bls12_381::G1Affine::deserialize_with_mode(cursor, Compress::No, Validate::No)
					.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_affine_g1(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = bls12_381::G1AffineOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_affine_g1_optimized(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = ark_bls12_381::G2Projective::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_projective_g2(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = bls12_381::G2ProjectiveOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_projective_g2_optimized(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base =
				ark_bls12_381::G2Affine::deserialize_with_mode(cursor, Compress::No, Validate::No)
					.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_affine_g2(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let cursor = Cursor::new(base);
			let base = bls12_381::G2AffineOptimized::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let cursor = Cursor::new(scalar);
			let scalar = u64::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let _ = crate::bls12_381::do_mul_affine_g2_optimized(&base, &[scalar]);
			Ok(())
		}

		#[pallet::call_index(16)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_prepare_inputs(_origin: OriginFor<T>, pvk: Vec<u8>) -> DispatchResult {
			let _inputs =
				crate::bls12_381::prepare_inputs_groth16::<ark_bls12_381::Bls12_381>(pvk).unwrap();
			Ok(())
		}

		#[pallet::call_index(17)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_prepare_inputs(
			_origin: OriginFor<T>,
			pvk: Vec<u8>,
		) -> DispatchResult {
			let _inputs =
				crate::bls12_381::prepare_inputs_groth16::<bls12_381::Bls12_381Optimized>(pvk)
					.unwrap();
			Ok(())
		}

		#[pallet::call_index(18)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verify_with_prepared_inputs(
			_origin: OriginFor<T>,
			inputs: Vec<u8>,
			pvk: Vec<u8>,
		) -> DispatchResult {
			crate::bls12_381::verify_with_prepared_inputs_groth16::<ark_bls12_381::Bls12_381>(
				inputs, pvk,
			)
			.unwrap();
			Ok(())
		}

		#[pallet::call_index(19)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_verify_with_prepared_inputs(
			_origin: OriginFor<T>,
			inputs: Vec<u8>,
			pvk: Vec<u8>,
		) -> DispatchResult {
			crate::bls12_381::verify_with_prepared_inputs_groth16::<bls12_381::Bls12_381Optimized>(
				inputs, pvk,
			)
			.unwrap();
			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_prepare_verifying_key(_origin: OriginFor<T>) -> DispatchResult {
			let _pvk =
				crate::bls12_381::prepare_verifying_key_groth16::<ark_bls12_381::Bls12_381>()
					.unwrap();
			Ok(())
		}

		#[pallet::call_index(21)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_optimized_prepare_verifying_key(_origin: OriginFor<T>) -> DispatchResult {
			let _pvk =
				crate::bls12_381::prepare_verifying_key_groth16::<bls12_381::Bls12_381Optimized>()
					.unwrap();
			Ok(())
		}
	}
}
