use std::marker::PhantomData;
use num::{
  Complex,
  Float,
};

use super::index_iterators::{
  Q1IdexIterator,
  Q2IdexIterator,
};

pub struct Q2Task<'a, T: 'a>
{
  pub(super) index_iter: Q2IdexIterator,
  pub(super) mut_ptr: *mut Complex<T>,
  pub(super) stride1: usize,
  pub(super) stride2: usize,
  pub(super) phantom: PhantomData<&'a T>,
}

unsafe impl<T> Send for Q2Task<'_, T> {}

impl<T> Q2Task<'_, T>
where
  T: Copy + Float + Default
{
  pub(super) fn matvec_inplace(self, matrix: &[Complex<T>]) {
    for idx in self.index_iter {
      let mut temp_arr: [Complex<T>; 4] = Default::default();
      for i in 0..2 {
        for j in 0..2 {
          for k in 0..2 {
            for l in 0..2 {
              unsafe {
                temp_arr[2 * i + j] = temp_arr[2 * i + j] + matrix[8 * i + 4 * j + 2 * k + l]
                                      * (*self.mut_ptr.add(self.stride1 * k + self.stride2 * l + idx));
              };
            }
          }
        }
      }
      for k in 0..2 {
        for l in 0..2 {
          unsafe { 
            *self.mut_ptr.add(self.stride1 * k + self.stride2 * l + idx) = temp_arr[2 * k + l];
          };
        }
      }
    }
  }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct Q1Task<'a, T: 'a>
{
  pub(super) index_iter: Q1IdexIterator,
  pub(super) mut_ptr: *mut Complex<T>,
  pub(super) stride: usize,
  pub(super) phantom: PhantomData<&'a T>,
}

unsafe impl<T> Send for Q1Task<'_, T> {}

impl<T> Q1Task<'_, T>
where
  T: Copy + Float + Default
{

  pub(super) fn matvec_inplace(self, matrix: &[Complex<T>]) {
    for idx in self.index_iter {
      let mut temp_arr: [Complex<T>; 4] = Default::default();
      for i in 0..2 {
        for j in 0..2 {
          unsafe {
            temp_arr[i] = temp_arr[i] + matrix[2 * i + j] * (*self.mut_ptr.add(self.stride * j + idx));
          }
        }
      }
      for i in 0..2 {
        unsafe { *self.mut_ptr.add(self.stride * i + idx) = temp_arr[i]; }
      }
    }
  }

  pub(super) fn partial_density_matrix_update(
    self,
    density_matrix: &mut [Complex<T>]
  )
  {
    for idx in self.index_iter {
      for i in 0..2 {
        for j in 0..2 {
          density_matrix[2 * i + j] = density_matrix[2 * i + j] + unsafe {
            (*self.mut_ptr.add(self.stride * i + idx))
            * (*self.mut_ptr.add(self.stride * j + idx)).conj()
          };
        }
      }
    }
  }
}
