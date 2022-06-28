use num::Complex;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::fs;

use crate::state_vector_machine::QStateBuilder;
use crate::utils::{get_local_density_matrices, execute};
use crate::layers::{get_5x5_heisenberg, qft};

pub fn heisenberg5x5() {
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
    let mut file = File::create("sigmaz_dynamics.txt").unwrap();

    let mut vec: Vec<Complex<f64>> = vec![Complex::new(0., 0.); 2usize.pow(25)];
    vec[2usize.pow(12)] = Complex::new(1., 0.);
    let mut state = QStateBuilder::new(vec).set_task_size(256).get_qstate();

    let start_time = Instant::now();
    for i in 0..layers_number {
        println!("Layer #{} is run", i);
        let layer = get_5x5_heisenberg(tau);
        for sigma_z in get_local_density_matrices(&mut state).map(|x| { x[0].re - x[3].re }) {
            file.write(&format!("{},", sigma_z).as_bytes()).unwrap();
        }
        execute(&mut state, layer);
    }
    for sigma_z in get_local_density_matrices(&mut state).map(|x| { x[0].re - x[3].re }) {
        file.write(&format!("{},", sigma_z).as_bytes()).unwrap();
    }
    let exec_time = start_time.elapsed().as_secs();
    println!("Total computation time is {} sec", exec_time);
    sleep(Duration::new(3, 0));
}

pub fn fourier_bench() {
  let qubits_numbers = [10, 14, 18, 22, 26, 28, 30];
  let task_size = 256;
  let mut file = fs::File::create("qft_benchmarks_log.txt").expect("Unable to create file for logging.");
  for qubits_number in qubits_numbers {
    let rng = rand::thread_rng();
    let mut state = QStateBuilder::new_random_unnormalized(qubits_number, rng)
      .set_task_size(task_size)
      .get_qstate();
    let qft_layer = qft::<f64>(qubits_number);
    let start_time = Instant::now();
    execute(&mut state, qft_layer);
    let exec_time = start_time.elapsed().as_nanos();
    let msg = format!("Total computation time for {} qubits is {} secs\n", qubits_number, (exec_time as f64) / 1e9);
    file.write(&msg.as_bytes()).expect(&format!("Unable to log time for {} qubits", qubits_number));
    println!("{}", msg);
  }
  sleep(Duration::new(3, 0));
}