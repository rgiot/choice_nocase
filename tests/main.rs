#[test]
fn ui() {
    let t = trybuild::TestCases::new();
	t.pass("tests/tests/standard.rs");
	t.pass("tests/tests/case_choice.rs");
	t.pass("tests/tests/byte_case_choice.rs");
	t.pass("tests/tests/case_choice2.rs");
	t.compile_fail("tests/tests/wrong_type.rs");
}