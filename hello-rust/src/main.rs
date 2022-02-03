use ferris_says::say;
use std::io::{BufWriter, stdout};
// use rand::Rng;
// use rand::prelude::random;
// use rand::prelude::thread_rng;
use rand::prelude::*;

fn main() {

	// let stdout = stdout();
	// let out = b"Injecting some Rust into my veins";
	// let width = 36;
	// let mut writer = BufWriter::new(stdout.lock());
	// say(out, width, &mut writer).unwrap();

	println!("{}", sqr(2.0));
	println!("{}", abs(2.0));
	println!("{}", clamp(9.0, 1.0, 5.0));

	/*
	 * You cannot print out an array in the usual way with {} 
	 * but you can do a debug print with {:?}.
	 *
	 * From https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
	 */
	let ints = [1, 2, 3];
	let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);

    // Rand stuff
    println!("\n---Random---");
    let x: u8 = random();
    println!("{}", x);
    let y: u32 = random();
    println!("{}", y);

    if random() { // generates a boolean
        println!("Heads!");
    }

    // If we want to be a bit more explicit (and a little more efficient) we can
    // make a handle to the thread-local generator:
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y = rng.gen_range(-10.0..10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
    }

    println!("Die roll: {}", rng.gen_range(1..=6));
    println!("Number from 0 to 9: {}", rng.gen_range(0..10));

}




/*
 * Demonstrating that the value of the last expression in a function 
 * is automatically returned, without using `return` explicitly.
 * 
 * From https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
 */
fn sqr(x: f64) -> f64 {
    x * x
}

/*
 * A few more examples of the no-return expression style:
 */

// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}