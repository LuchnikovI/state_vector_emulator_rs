#![feature(test)]
extern crate test;

pub mod state_vector_machine;
pub mod gates;
//pub mod utils;
//pub mod layers;

use state_vector_machine::{*, QStateBuilder};
use test::Bencher;
use num::{Zero, One, Complex};

macro_rules! q2_gate_bench {
  ($num_threads:expr, $num_qubits:expr, $task_size:expr, $name:ident) => {
    #[bench]
    fn $name(b: &mut Bencher) {
      let number_of_qubits = $num_qubits;
      let mut indices_iter = (0..number_of_qubits).flat_map(|x| {
          (0..number_of_qubits).map(move |y| { (x, y)})
      }).filter(|(x, y)| { x != y }).cycle();
      let threads_number = $num_threads;
      let task_size = $task_size;
      let gate = Heisenberg!(f64, 0.3);
      let mut state = QStateBuilder::<f64>::new_standard(number_of_qubits)
        .set_task_size(task_size)
        .set_threads_number(threads_number)
        .get_qstate();
      b.iter(|| {
        let (i, j) = indices_iter.next().unwrap();
        state.apply_2q_gate(&gate, i, j);
      })
    }
  };
}

q2_gate_bench!(1, 20, 16, qubits_number_20_threads_number_1_task_size_16);
q2_gate_bench!(2, 20, 16, qubits_number_20_threads_number_2_task_size_16);
q2_gate_bench!(4, 20, 16, qubits_number_20_threads_number_4_task_size_16);
q2_gate_bench!(6, 20, 16, qubits_number_20_threads_number_6_task_size_16);
q2_gate_bench!(8, 20, 16, qubits_number_20_threads_number_8_task_size_16);
q2_gate_bench!(10, 20, 16, qubits_number_20_threads_number__10_task_size_16);
q2_gate_bench!(12, 20, 16, qubits_number_20_threads_number__12_task_size_16);
q2_gate_bench!(14, 20, 16, qubits_number_20_threads_number__14_task_size_16);
q2_gate_bench!(16, 20, 16, qubits_number_20_threads_number__16_task_size_16);
q2_gate_bench!(1, 25, 16, qubits_number_25_threads_number_1_task_size_16);
q2_gate_bench!(2, 25, 16, qubits_number_25_threads_number_2_task_size_16);
q2_gate_bench!(4, 25, 16, qubits_number_25_threads_number_4_task_size_16);
q2_gate_bench!(6, 25, 16, qubits_number_25_threads_number_6_task_size_16);
q2_gate_bench!(8, 25, 16, qubits_number_25_threads_number_8_task_size_16);
q2_gate_bench!(10, 25, 16, qubits_number_25_threads_number__10_task_size_16);
q2_gate_bench!(12, 25, 16, qubits_number_25_threads_number__12_task_size_16);
q2_gate_bench!(14, 25, 16, qubits_number_25_threads_number__14_task_size_16);
q2_gate_bench!(16, 25, 16, qubits_number_25_threads_number__16_task_size_16);
q2_gate_bench!(1, 20, 256, qubits_number_20_threads_number_1_task_size_256);
q2_gate_bench!(2, 20, 256, qubits_number_20_threads_number_2_task_size_256);
q2_gate_bench!(4, 20, 256, qubits_number_20_threads_number_4_task_size_256);
q2_gate_bench!(6, 20, 256, qubits_number_20_threads_number_6_task_size_256);
q2_gate_bench!(8, 20, 256, qubits_number_20_threads_number_8_task_size_256);
q2_gate_bench!(10, 20, 256, qubits_number_20_threads_number__10_task_size_256);
q2_gate_bench!(12, 20, 256, qubits_number_20_threads_number__12_task_size_256);
q2_gate_bench!(14, 20, 256, qubits_number_20_threads_number__14_task_size_256);
q2_gate_bench!(16, 20, 256, qubits_number_20_threads_number__16_task_size_256);
q2_gate_bench!(1, 25, 256, qubits_number_25_threads_number_1_task_size_256);
q2_gate_bench!(2, 25, 256, qubits_number_25_threads_number_2_task_size_256);
q2_gate_bench!(4, 25, 256, qubits_number_25_threads_number_4_task_size_256);
q2_gate_bench!(6, 25, 256, qubits_number_25_threads_number_6_task_size_256);
q2_gate_bench!(8, 25, 256, qubits_number_25_threads_number_8_task_size_256);
q2_gate_bench!(10, 25, 256, qubits_number_25_threads_number__10_task_size_256);
q2_gate_bench!(12, 25, 256, qubits_number_25_threads_number__12_task_size_256);
q2_gate_bench!(14, 25, 256, qubits_number_25_threads_number__14_task_size_256);
q2_gate_bench!(16, 25, 256, qubits_number_25_threads_number__16_task_size_256);
q2_gate_bench!(1, 20, 1024, qubits_number_20_threads_number_1_task_size_1024);
q2_gate_bench!(2, 20, 1024, qubits_number_20_threads_number_2_task_size_1024);
q2_gate_bench!(4, 20, 1024, qubits_number_20_threads_number_4_task_size_1024);
q2_gate_bench!(6, 20, 1024, qubits_number_20_threads_number_6_task_size_1024);
q2_gate_bench!(8, 20, 1024, qubits_number_20_threads_number_8_task_size_1024);
q2_gate_bench!(10, 20, 1024, qubits_number_20_threads_number__10_task_size_1024);
q2_gate_bench!(12, 20, 1024, qubits_number_20_threads_number__12_task_size_1024);
q2_gate_bench!(14, 20, 1024, qubits_number_20_threads_number__14_task_size_1024);
q2_gate_bench!(16, 20, 1024, qubits_number_20_threads_number__16_task_size_1024);
q2_gate_bench!(1, 25, 1024, qubits_number_25_threads_number_1_task_size_1024);
q2_gate_bench!(2, 25, 1024, qubits_number_25_threads_number_2_task_size_1024);
q2_gate_bench!(4, 25, 1024, qubits_number_25_threads_number_4_task_size_1024);
q2_gate_bench!(6, 25, 1024, qubits_number_25_threads_number_6_task_size_1024);
q2_gate_bench!(8, 25, 1024, qubits_number_25_threads_number_8_task_size_1024);
q2_gate_bench!(10, 25, 1024, qubits_number_25_threads_number__10_task_size_1024);
q2_gate_bench!(12, 25, 1024, qubits_number_25_threads_number__12_task_size_1024);
q2_gate_bench!(14, 25, 1024, qubits_number_25_threads_number__14_task_size_1024);
q2_gate_bench!(16, 25, 1024, qubits_number_25_threads_number__16_task_size_1024);
