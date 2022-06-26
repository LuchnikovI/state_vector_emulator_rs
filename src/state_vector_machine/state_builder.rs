use num::{Complex, Float};

use super::state::QState;

/////////////////////////////////////////////////////////////////
fn is_pow_of_2(number: usize) -> bool {
  if number == 0 { return false };
  if number & (number - 1) != 0 { false } else { true }
}

fn get_qubits_number(mut state_size: usize) -> usize {
  let mut result = 0;
  while state_size > 1 {
    state_size >>= 1;
    result += 1;
  }
  result
}

pub struct QStateBuilder<T: Float + Clone + Default + Sync + Send> {
  state: Vec<Complex<T>>,
  qubits_number: usize,
  task_size: Option<usize>,
}

impl <T: Float + Clone + Default + Sync + Send> QStateBuilder<T> {
  pub fn new(vec: Vec<Complex<T>>) -> Self {
    if is_pow_of_2(vec.len()) {
      let qubits_number = get_qubits_number(vec.len());
      QStateBuilder {
        state: vec,
        qubits_number: qubits_number,
        task_size: None,
      }
    } else {
      panic!("Size of a state is not a power of 2.");
    }
  }
  
  pub fn new_standard(qubits_number: usize) -> Self {
    let size = 2usize.pow(qubits_number as u32);
    let mut vec = vec![Complex::new(T::zero(), T::zero()); size];
    vec[0] = Complex::new(T::one(), T::zero());
    QStateBuilder {
      state: vec,
      qubits_number: qubits_number,
      task_size: None,
    }
  }

  pub fn set_task_size(mut self, size: usize) -> Self {
    assert!(is_pow_of_2(size), "Task size must be a power of 2.");
    assert!(get_qubits_number(size) <= self.qubits_number - 2, "Taks size must be less ot equal to the number of qubits - 2.");
    self.task_size = Some(size);
    self
  }

  pub fn get_qstate(self) -> QState<T> {
    if let Some(s) = self.task_size {
      QState {
        state: self.state,
        qubits_number: self.qubits_number,
        task_size: s,
      }
    } else { panic!("Task size has not been set.") }
  }
}