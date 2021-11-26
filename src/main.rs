mod core;

use crate::core::blockchain::Blockchain;

fn main() {
    let bc = Blockchain::new();

    println!("bc :{:?}", bc)
}
