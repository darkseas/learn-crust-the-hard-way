fn main() {
	let s = "a";

	match s {
		&"a" => println!("found a"),
		&"b" => println!("found b"),
		_ => println!("not here")
	}
}