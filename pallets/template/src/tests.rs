use crate::{bls12_381, mock::*};
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
