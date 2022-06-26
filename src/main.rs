pub mod state_vector_machine;
pub mod gates;
pub mod utils;
pub mod layers;

use num::{
    Complex,
    Zero,
    One,
};
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::time::{Instant, Duration};
use std::thread::sleep;

use state_vector_machine::QStateBuilder;
use utils::{QInstruction, get_local_density_matrices, execute};

fn main() {
    println!("Enter discretization parameter tau:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("io error.");
    let tau = buffer.trim().parse::<f64>().expect("parsing error.");
    assert!(tau > 0., "tau must be > 0.");
    println!("Enter number of layers:");
    buffer.clear();
    stdin().read_line(&mut buffer).expect("io error.");
    let layers_number = buffer.trim().parse::<usize>().expect("parsing error.");
    println!("Discretization parameter is {}\nNumber of layers is {}", tau, layers_number);
    let layer = Heisenberg5X5!(f64, tau);
    let mut file = File::create("sigmaz_dynamics.txt").unwrap();

    let mut vec: Vec<Complex<f64>> = vec![Complex::new(0., 0.); 2usize.pow(25)];
    vec[2usize.pow(12)] = Complex::new(1., 0.);
    let mut state = QStateBuilder::new(vec).set_task_size(256).get_qstate();

    let start_time = Instant::now();
    for i in 0..layers_number {
        println!("Layer #{} is run", i);
        for sigma_z in get_local_density_matrices(&mut state).map(|x| { x[0].re - x[3].re }) {
            file.write(&format!("{},", sigma_z).as_bytes()).unwrap();
        }
        execute(&mut state, layer.iter());
    }
    for sigma_z in get_local_density_matrices(&mut state).map(|x| { x[0].re - x[3].re }) {
        file.write(&format!("{},", sigma_z).as_bytes()).unwrap();
    }
    let exec_time = start_time.elapsed().as_secs();
    println!("Total computation time is {} sec", exec_time);
    sleep(Duration::new(3, 0));
}
