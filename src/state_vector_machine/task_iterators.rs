use std::marker::PhantomData;
use num::{
  Complex,
  Float,
};

use super::state::QState;
use super::tasks::{
  Q1Task,
  Q2Task,
};
use super::index_iterators::{
  Q1IdexIterator,
  Q2IdexIterator,
};

///////////////////////////////////////////////////////////////////////

pub struct Q2TasksIterator<'a, T>
where
  T: Float + 'a,
{
  idx1: usize,
  idx2: usize,
  stride1: usize,
  stride2: usize,
  task_size: usize,
  state: usize,
  end: usize,
  mut_ptr: *mut Complex<T>,
  phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Q2TasksIterator<'a, T>
where
  T: Float + Clone + Default + Sync + Send + 'a,
{
  pub(super) fn new(
    state: &mut QState<T>,
    idx1: usize,
    idx2: usize,
    task_size: usize,
  ) -> Self {
    let stride1 = 2usize.pow(idx1 as u32);
    let stride2 = 2usize.pow(idx2 as u32);
    let end = 2usize.pow((state.get_qubits_number() - 2) as u32);
    Q2TasksIterator {
      idx1: idx1,
      idx2: idx2,
      stride1: stride1,
      stride2: stride2,
      task_size: task_size,
      state: 0,
      end: end,
      mut_ptr: state.state.as_mut_ptr(),
      phantom: PhantomData,
    }
  }
}

impl<'a, T> Iterator for Q2TasksIterator<'a, T>
where
  T: Float + 'a,
{
  type Item = Q2Task<'a, T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.state >= self.end {
      None
    } else {
      let task_start = self.state;
      self.state += self.task_size;
      let task_index_iter = Q2IdexIterator::new(self.idx1, self.idx2, task_start, self.task_size);
      Some(Q2Task {
        index_iter: task_index_iter,
        mut_ptr: self.mut_ptr,
        stride1: self.stride1,
        stride2: self.stride2,
        phantom: PhantomData,
      })
    }
  }
}

///////////////////////////////////////////////////////////////////////

pub struct Q1TasksIterator<'a, T>
where
  T: Float + 'a,
{
  idx: usize,
  stride: usize,
  task_size: usize,
  state: usize,
  end: usize,
  mut_ptr: *mut Complex<T>,
  phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Q1TasksIterator<'a, T>
where
  T: Float + Clone + Default + Sync + Send + 'a,
{
  pub(super) fn new(
    state: &mut QState<T>,
    idx: usize,
    task_size: usize,
  ) -> Self {
    let stride = 2usize.pow(idx as u32);
    let end = 2usize.pow((state.get_qubits_number() - 1) as u32);
    Q1TasksIterator {
      idx: idx,
      stride: stride,
      task_size: task_size,
      state: 0,
      end: end,
      mut_ptr: state.state.as_mut_ptr(),
      phantom: PhantomData,
    }
  }
}

impl<'a, T> Iterator for Q1TasksIterator<'a, T>
where
  T: Float + 'a,
{
  type Item = Q1Task<'a, T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.state >= self.end {
      None
    } else {
      let task_start = self.state;
      self.state += self.task_size;
      let task_index_iter = Q1IdexIterator::new(self.idx, task_start, self.task_size);
      Some(Q1Task {
        index_iter: task_index_iter,
        mut_ptr: self.mut_ptr,
        stride: self.stride,
        phantom: PhantomData,
      })
    }
  }
}