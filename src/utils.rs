use num::{Float, Complex};

use crate::state_vector_machine::QState;

#[derive(Debug)]
pub enum QInstruction<T>
where
  T: Float + Clone + Default + Sync + Send
{
  Q1 {
    idx: usize,
    gate: [Complex<T>; 4]
  },
  Q2 {
    idx1: usize,
    idx2: usize,
    gate: [Complex<T>; 16]
  }
}

impl<T> QInstruction<T>
where
  T: Float + Clone + Default + Sync + Send
{
  pub fn apply(&self, state: &mut QState<T>) {
    match self {
      QInstruction::Q1 { idx, gate } => {
        state.apply_1q_gate(&gate[..], *idx)
      },
      QInstruction::Q2 { idx1, idx2, gate } => {
        state.apply_2q_gate(&gate[..], *idx1, *idx2)
      },
    };
  }
}

pub fn execute<T>(
  state: &mut QState<T>,
  instructions: impl IntoIterator<Item=QInstruction<T>>
)
where
  T: Float + Clone + Default + Sync + Send
{
  instructions.into_iter().for_each(|instruction| {
    instruction.apply(state);
  });
}

pub fn get_local_density_matrices<T>(state: &mut QState<T>) -> impl Iterator<Item=[Complex<T>; 4]> + '_
where
  T: Float + Clone + Default + Sync + Send
{
  let qubits_number = state.get_qubits_number();
  (0..qubits_number).map(|i| {
    state.get_1q_partial_density_matrix(i)
  })
}