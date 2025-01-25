// Declaring module 
pub mod garden;

// importing sub module 
#[allow(unused_imports)]
use garden::vegetable::Weed;

fn main() {
    println!("Hello, world!");
}
