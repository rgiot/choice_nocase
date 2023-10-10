fn standard_case() {
	let value = "HI".to_owned();
	match value.as_ref() {
		"HI" | "hi" | "Hi" | "hI" => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}