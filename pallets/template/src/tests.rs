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
