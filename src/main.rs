//!
//! application entrypoint.
//!

extern crate uuid;

fn main() {
    
    let uuid = uuid::Uuid::new_v4();
	println!("{}", uuid);
}
