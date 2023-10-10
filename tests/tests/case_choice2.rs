use choice_nocase::choice_nocase;

fn standard_case() {
	let value = "hello".to_owned();
	match value.as_ref() {
		choice_nocase!("hello") => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}