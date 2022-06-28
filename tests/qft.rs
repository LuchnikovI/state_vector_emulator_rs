use std::iter::zip;
use dft::{Operation, Plan, c64};

use state_vector_emulator_rs::{
  state_vector_machine::QStateBuilder,
  layers::qft,
  layers::reverse,
  utils::execute,
};

#[test]
fn qft_test () {
  let qubits_number = 15;
  let task_size = 128;
  let rng = rand::thread_rng();
  let mut state = QStateBuilder::new_random_unnormalized(qubits_number, rng)
    .set_task_size(task_size)
    .get_qstate();
  /*let mut state = QStateBuilder::new_standard(qubits_number)
    .set_task_size(task_size)
    .get_qstate();*/
  /*let mut vec: Vec<Complex<f64>> = vec![Complex::new(0., 0.); 2usize.pow(qubits_number as u32)];
  vec[1] = Complex::new(1., 0.);
  let mut state = QStateBuilder::new(vec).set_task_size(task_size).get_qstate();*/

  let mut vec: Vec<c64> = state.get_copy_vec()
    .into_iter()
    .map(|x| { c64::new(x.re, x.im) })
    .collect();

  let reverse_layer = reverse::<f64>(qubits_number);
  let qft_layer = qft::<f64>(qubits_number);
  /*for instruction in qft_layer {
    println!("{:?}", instruction);
  }*/
  execute(&mut state, reverse_layer);
  execute(&mut state, qft_layer);

  let plan = Plan::new(Operation::Inverse, 2usize.pow(qubits_number as u32));
  dft::transform(&mut vec, &plan);
  let flag = zip(state.release_vec().into_iter(), vec.into_iter())
    .all(|(lhs, rhs)| {
      let diff = c64::new(lhs.re, lhs.im) - rhs * (2f64.powi(qubits_number as i32)).sqrt();
      c64::norm(&diff) < 1e-10
    });
  assert!(flag);
}