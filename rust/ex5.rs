// This should be the idiomatic rust structure

use std::io;

// main function
fn main()  {
	let prompt = "--->";
	let mut i = 1;

	println!("To exit type Ctrl-D");
	for line in io::stdin().lines() {
		let mut token = ~ "--";
		match line {
			Ok(m)  => {
				token = my_parse(m.trim());
				handle(token);
			}
			Err(e) => println!("Failed: {}", e)
		}
		if token == ~"exit" {
			break;
		}
		i += 1;

	}

}

fn my_parse(msg: &str) -> ~str {
	println!("my_parse has got {}", msg);
	let mut token: ~str;
	match msg {
		&"a"    => ~"a",
		&"exit" => ~"exit",
		_       => ~"catchall"
	}
}

fn handle(token: &str) {
	println!("handle has {}", token);
}
