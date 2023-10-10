use choice_nocase::choice_nocase;

fn standard_case() {
	let value = "HI".to_owned();
	match value.as_ref() {
		choice_nocase!("hi") => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}