use choice_nocase::choice_nocase;

fn standard_case() {
	let value = b"HI";
	match value {
		choice_nocase!(b"hi") => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}