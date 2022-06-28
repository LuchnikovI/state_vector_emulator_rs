pub mod state_vector_machine;
pub mod gates;
pub mod utils;
pub mod layers;
pub mod examples;

use std::io::stdin;

use examples::{
    heisenberg5x5,
    fourier_bench,
};

fn main() {
    println!("Enter the number of an example from the list: \n 
              1: Heisenber 5x5 \n 
              2: Quantum fourier transform");
    let mut example = String::new();
    stdin().read_line(&mut example).expect("Io error.");
    let number = example.trim().parse::<u32>().expect("Fail parsing to a number.");
    match number {
        1 => { heisenberg5x5() },
        2 => { fourier_bench() },
        _ => {panic!("Incorrect example number.")}
    }
}
