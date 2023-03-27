use crate::{
	bls12_377, bls12_381, bw6_761, ed_on_bls12_377, ed_on_bls12_381,
	mock::*,
	utils::{generate_msm_args, generate_pairing_args, generate_scalar_args},
};
use frame_support::assert_ok;

#[test]
fn groth16_verification() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::groth16_verification(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn groth16_verificaton_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::groth16_verification_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381() {
	new_test_ext().execute_with(|| {
		let (a, b) = generate_pairing_args::<
			<ark_ec::bls12::Bls12<ark_bls12_381::Config> as ark_ec::pairing::Pairing>::G1Affine,
			<ark_ec::bls12::Bls12<ark_bls12_381::Config> as ark_ec::pairing::Pairing>::G2Affine,
		>();
		assert_ok!(TemplateModule::bls12_381_pairing(RuntimeOrigin::signed(1), a, b));
	});
}
#[test]
fn pairing_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (a, b) =
			generate_pairing_args::<bls12_381::G1AffineOptimized, bls12_381::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_pairing_optimized(RuntimeOrigin::signed(1), a, b));
	});
}
#[test]
fn msm_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			sp_ark_models::short_weierstrass::Projective<
				sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>,
			>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			sp_ark_models::short_weierstrass::Projective<
				sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>,
			>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G1ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
#[test]
fn mul_projective_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G1Projective>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1_optimized(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
#[test]
fn mul_affine_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G1Affine>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1(RuntimeOrigin::signed(1), base, scalar));
	});
}
#[test]
fn mul_affine_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G1AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1_optimized(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
#[test]
fn mul_projective_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G2Projective>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
#[test]
fn mul_projective_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G2ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2_optimized(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
#[test]
fn mul_affine_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2(RuntimeOrigin::signed(1), base, scalar));
	});
}
#[test]
fn mul_affine_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G2Affine>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2_optimized(
			RuntimeOrigin::signed(1),
			base,
			scalar
		));
	});
}
