//!
//! application entrypoint.
//!

extern crate uuid;

fn main() {

	// That's all!
	let uuid = uuid::Uuid::new_v4();
	println!("{}", uuid);
}
