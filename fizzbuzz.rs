fn	main() {
	let mut count = 1;

	loop {
		if count > 100 {
			break;
		} else if count % 15 == 0 {
			println!("FizzBuzz");
		} else if count % 3 == 0 {
			println!("Fizz");
		} else if count % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", count);
		}
		count += 1;
	}
}
