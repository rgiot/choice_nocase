use choice_nocase::case_choice;

fn standard_case() {
	let value = "HI".to_owned();
	match value.as_ref() {
		case_choice!("hi") => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}