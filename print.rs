#[derive(Debug)]

struct Person<'a> {
	name: &'a str,
	age: u8
}

fn	main() {
	println!("{} days", 31);
	println!("{0}, this is {1}, {1} this is {0}", "Alice", "Bob");
	println!("{subject} {verb} {object}",
		object="the lazy dog",
		subject="the quick brown fox",
		verb="jumps over");	
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
	println!("{number:>width$}", number=1, width=6);

	#[allow(dead_code)]
	struct Structure(i32);
	
	// println!("This struct '{}' wont print...", Structure(3));

	let pi = 3.141592;
	println!("{:.3}", pi);

	let name = "Peter";
	let age = 27;
	let peter = Person { name, age};

	println!("{:#?}", peter);
}
