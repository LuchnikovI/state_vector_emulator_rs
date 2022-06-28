use num::Float;
use std::iter::from_fn;
use std::f64::consts::PI;

use crate::gates::{
  get_partial_swap,
  get_hadamard,
  get_cphase,
  get_swap,
};
use crate::utils::QInstruction;

// Periodic boundary conditions
pub fn get_5x5_heisenberg<T>(tau: T) -> impl IntoIterator<Item = QInstruction<T>>
where
  T: Float + Default + Send + Sync
{
  let gate = get_partial_swap(tau);
  [
    QInstruction::Q2 { idx1: 0, idx2: 5, gate: gate },
    QInstruction::Q2 { idx1: 1, idx2: 6, gate: gate },
    QInstruction::Q2 { idx1: 2, idx2: 7, gate: gate },
    QInstruction::Q2 { idx1: 3, idx2: 8, gate: gate },
    QInstruction::Q2 { idx1: 4, idx2: 9, gate: gate },
    QInstruction::Q2 { idx1: 10, idx2: 15, gate: gate },
    QInstruction::Q2 { idx1: 11, idx2: 16, gate: gate },
    QInstruction::Q2 { idx1: 12, idx2: 17, gate: gate },
    QInstruction::Q2 { idx1: 13, idx2: 18, gate: gate },
    QInstruction::Q2 { idx1: 14, idx2: 19, gate: gate },
    QInstruction::Q2 { idx1: 5, idx2: 10, gate: gate },
    QInstruction::Q2 { idx1: 6, idx2: 11, gate: gate },
    QInstruction::Q2 { idx1: 7, idx2: 12, gate: gate },
    QInstruction::Q2 { idx1: 8, idx2: 13, gate: gate },
    QInstruction::Q2 { idx1: 9, idx2: 14, gate: gate },
    QInstruction::Q2 { idx1: 15, idx2: 20, gate: gate },
    QInstruction::Q2 { idx1: 16, idx2: 21, gate: gate },
    QInstruction::Q2 { idx1: 17, idx2: 22, gate: gate },
    QInstruction::Q2 { idx1: 18, idx2: 23, gate: gate },
    QInstruction::Q2 { idx1: 19, idx2: 24, gate: gate },
    QInstruction::Q2 { idx1: 0, idx2: 1, gate: gate },
    QInstruction::Q2 { idx1: 5, idx2: 6, gate: gate },
    QInstruction::Q2 { idx1: 10, idx2: 11, gate: gate },
    QInstruction::Q2 { idx1: 15, idx2: 16, gate: gate },
    QInstruction::Q2 { idx1: 20, idx2: 21, gate: gate },
    QInstruction::Q2 { idx1: 2, idx2: 3, gate: gate },
    QInstruction::Q2 { idx1: 7, idx2: 8, gate: gate },
    QInstruction::Q2 { idx1: 12, idx2: 13, gate: gate },
    QInstruction::Q2 { idx1: 17, idx2: 18, gate: gate },
    QInstruction::Q2 { idx1: 22, idx2: 23, gate: gate },
    QInstruction::Q2 { idx1: 1, idx2: 2, gate: gate },
    QInstruction::Q2 { idx1: 6, idx2: 7, gate: gate },
    QInstruction::Q2 { idx1: 11, idx2: 12, gate: gate },
    QInstruction::Q2 { idx1: 16, idx2: 17, gate: gate },
    QInstruction::Q2 { idx1: 21, idx2: 22, gate: gate },
    QInstruction::Q2 { idx1: 3, idx2: 4, gate: gate },
    QInstruction::Q2 { idx1: 8, idx2: 9, gate: gate },
    QInstruction::Q2 { idx1: 13, idx2: 14, gate: gate },
    QInstruction::Q2 { idx1: 18, idx2: 19, gate: gate },
    QInstruction::Q2 { idx1: 23, idx2: 24, gate: gate },
    QInstruction::Q2 { idx1: 0, idx2: 4, gate: gate },
    QInstruction::Q2 { idx1: 5, idx2: 9, gate: gate },
    QInstruction::Q2 { idx1: 10, idx2: 14, gate: gate },
    QInstruction::Q2 { idx1: 15, idx2: 19, gate: gate },
    QInstruction::Q2 { idx1: 20, idx2: 24, gate: gate },
    QInstruction::Q2 { idx1: 0, idx2: 20, gate: gate },
    QInstruction::Q2 { idx1: 1, idx2: 21, gate: gate },
    QInstruction::Q2 { idx1: 2, idx2: 22, gate: gate },
    QInstruction::Q2 { idx1: 3, idx2: 23, gate: gate },
    QInstruction::Q2 { idx1: 4, idx2: 24, gate: gate },
  ]
}

pub fn qft<T>(
  total_qubits_number: usize
) -> impl IntoIterator<Item = QInstruction<T>>
where
  T: Float + Clone + Default + Sync + Send
{
  (0..total_qubits_number).flat_map(move |qubit_number| {
    (qubit_number..total_qubits_number).map(move |i| {
      if i == qubit_number {
        QInstruction::Q1 { idx: qubit_number, gate: get_hadamard()}
      } else {
        QInstruction::Q2 {
          idx1: i,
          idx2: qubit_number,
          gate: get_cphase(T::from(PI / 2f64.powi((i - qubit_number) as i32)).unwrap()) }
      }
    })
  })
}

pub fn reverse<T>(
  total_qubits_number: usize
) -> impl IntoIterator<Item = QInstruction<T>>
where
  T: Float + Clone + Default + Sync + Send
{
  let swap = get_swap();
  let mut start: i32 = 0;
  let mut end: i32 = (total_qubits_number-1) as i32;
  from_fn(move || {
    if end - start > 0 {
      let curr_start = start;
      let curr_end = end;
      start += 1;
      end -= 1;
      Some(QInstruction::Q2 {
        idx1: curr_start as usize,
        idx2: curr_end as usize,
        gate: swap
      })
    } else {
      None
    }
  })
}
