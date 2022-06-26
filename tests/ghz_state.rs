use num::{*, complex::ComplexFloat};
use std::iter::zip;

use simple_state_vec_emulator::{
  state_vector_machine::QStateBuilder,
  Hadamard,
  CX,
};

#[test]
fn ghz_state_test() {
  let qubits_number = 20;
  let task_size = 256;
  let mut state = QStateBuilder::new_standard(qubits_number)
    .set_task_size(task_size)
    .get_qstate();
  let h_gate = Hadamard!(f32);
  let cx_gate = CX!(f32);
  state.apply_1q_gate(&h_gate, 0);
  for i in 0..(qubits_number-1) {
    state.apply_2q_gate(&cx_gate, i, i+1);
  }
  for i in 0..qubits_number {
    let local_dens = state.get_1q_partial_density_matrix(i);
    let true_local_dens = [
      Complex::new(0.5, 0.),
      Complex::new(0., 0.),
      Complex::new(0., 0.),
      Complex::new(0.5, 0.),
    ];
    let flag = zip(local_dens.into_iter(), true_local_dens.into_iter())
      .all(|(lhs, rhs)| {Complex::abs(lhs - rhs) < 1e-5});
    assert!(flag);
  }
  let state_size = 2.pow(qubits_number as u32);
  let mut ghz_state = vec![Complex::new(0., 0.); state_size];
  ghz_state[0] = Complex::new(1. / num::Float::sqrt(2.), 0.);
  ghz_state[state_size-1] = Complex::new(1. / num::Float::sqrt(2.), 0.);
  let flag = zip(ghz_state.into_iter(), state.release_vec().into_iter())
    .all(|(lhs, rhs)| {
      (lhs - rhs).abs() < 1e-5
    });
  assert!(flag);
}