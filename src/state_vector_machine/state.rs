use num::{Complex, Float};
use std::fmt::Debug;
use crossbeam::{
  channel::bounded,
  scope,
};
use std::iter::zip;

use super::tasks::{
  Q1Task,
  Q2Task,
};
use super::task_iterators::{
  Q1TasksIterator,
  Q2TasksIterator,
};


#[derive(Debug)]
pub struct QState<T: Float + Clone + Default + Sync + Send> {
  pub(super) state: Vec<Complex<T>>,
  pub(super) qubits_number: usize,
  pub(super) task_size: usize,
  pub(super) threads_number: usize,
}

impl<T: Float + Clone + Default + Sync + Send> QState<T> {

  pub fn release_vec(self) -> Vec<Complex<T>> {
    self.state
  }

  pub fn get_copy_vec(&self) -> Vec<Complex<T>> {
    self.state.clone()
  }

  pub fn get_qubits_number(&self) -> usize { self.qubits_number }
}

/////////////////////////////////////////////////////////////////////////////

impl<T: Float + Clone + Default + Sync + Send> QState<T> {

  pub fn apply_2q_gate(
    &mut self,
    gate: &[Complex<T>],
    idx1: usize,
    idx2: usize,
  )
  {
    let threads_number = self.threads_number;
    let task_iter = Q2TasksIterator::new(self, idx1, idx2, self.task_size);
    let(sn, rs) = bounded::<Q2Task<T>>(threads_number);
    scope(|s| {
      for _ in 0..threads_number {
        let local_rs = rs.clone();
        let gate_ref = &gate[..];
        s.spawn(move |_| {
          loop {
            let task = local_rs.recv();
            match task {
              Result::Err(..) => { break },
              Ok(q2task) => {
                q2task.matvec_inplace(gate_ref);
              },
            };
          }
        });
      }
      task_iter.for_each(|x| {
        sn.send(x).unwrap();
      });
      drop(sn);
    }).unwrap();
  }

  pub fn apply_1q_gate(
    &mut self,
    gate: &[Complex<T>],
    idx: usize,
  )
  {
    let threads_number = self.threads_number;
    let task_iter = Q1TasksIterator::new(self, idx, self.task_size);
    let(sn, rs) = bounded::<Q1Task<T>>(threads_number);
    scope(|s| {
      for _ in 0..threads_number {
        let local_rs = rs.clone();
        let gate_ref = &gate[..];
        s.spawn(move |_| {
          loop {
            let task = local_rs.recv();
            match task {
              Result::Err(..) => { break },
              Ok(q1task) => {
                q1task.matvec_inplace(gate_ref);
              },
            };
          }
        });
      }
      task_iter.for_each(|x| {
        sn.send(x).unwrap();
      });
      drop(sn);
    }).unwrap();
  }

  pub fn get_1q_partial_density_matrix(
    &mut self,
    idx: usize,
  ) -> [Complex<T>; 4]
  {
    let threads_number = self.threads_number;
    let task_iter = Q1TasksIterator::new(self, idx, self.task_size);
    let(sn, rs) = bounded::<Q1Task<T>>(threads_number);
    let (dens_sender, dens_reciver) = bounded::<[Complex<T>; 4]>(threads_number);
    scope(|s| {
      for _ in 0..threads_number {
        let local_rs = rs.clone();
        let local_dens_sender = dens_sender.clone();
        let mut density_matrix: [Complex<T>; 4] = Default::default();
        s.spawn(move |_| {
          loop {
            let task = local_rs.recv();
            match task {
              Result::Err(..) => {
                local_dens_sender.send(density_matrix).unwrap();
                break;
              },
              Ok(q1task) => {
                q1task.partial_density_matrix_update(&mut density_matrix[..]);
              },
            };
          }
        });
      }
      task_iter.for_each(|x| {
        sn.send(x).unwrap();
      });
      drop(sn);
    }).unwrap();
    drop(dens_sender);
    let mut density_matrix: [Complex<T>; 4] = Default::default();
    for mut dens in dens_reciver {
      zip(dens.iter_mut(), density_matrix.iter_mut())
        .for_each(|(lhs, rhs)| {
          *rhs = *lhs + *rhs;
        })
    };
    density_matrix
  }
}
