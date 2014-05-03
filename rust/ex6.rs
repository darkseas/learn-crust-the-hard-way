// ex6.rs
// Value literals

fn main() {
	
	let distance = 100i;
	let power = 2.345f32;
	let super_power = 56789.4532f64;
	let initial = 'J';
	let first_name = "Gav";
	let last_name = "Coombes";

	println!("You are {:d} miles away.", distance);
	println!("You have {:f} levels of power.", power);
	println!("You have {:f} awesome super powers.", super_power);
	println!("I have an initial {:c}.", initial);
	println!("I have a first name {:s}.", first_name);
	println!("I have a last name {:s}.", last_name);
	println!("My whole name is {:s} {:c}. {:s}.", 
		first_name, initial, last_name);
}



