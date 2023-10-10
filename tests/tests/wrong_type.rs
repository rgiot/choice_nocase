use choice_nocase::case_choice;


fn standard_case() {
	let value = 2;
	match value {
		case_choice!(2) => println!("good"),
		_ => println!("bad")
	};
}

fn main() {
	standard_case()
}