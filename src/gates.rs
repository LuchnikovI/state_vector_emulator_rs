use num::{Complex, Float};

pub fn get_hadamard<T: Float>() -> [Complex<T>; 4]
{
  let elem = T::from(1f64 / 2f64.sqrt()).unwrap();
  [
    Complex::new(elem, T::zero()),
    Complex::new(elem, T::zero()),
    Complex::new(elem, T::zero()),
    Complex::new(-elem, T::zero()),
  ]
}

pub fn get_phase<T: Float>(phi: T) -> [Complex<T>; 4]
{
  [
    Complex::new(T::one(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(phi.cos(), phi.sin()),
  ]
}

pub fn get_cphase<T: Float>(phi: T) -> [Complex<T>; 16]
{
  [
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(phi.cos(), phi.sin()),
  ]
}

pub fn get_cx<T: Float>() -> [Complex<T>; 16]
{
  [
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::zero(), T::zero()),
    Complex::new(T::one(), T::zero()),
    Complex::new(T::zero(), T::zero()),
  ]
}

pub fn get_swap<T: Float>() -> [Complex<T>; 16]
{
  [
      Complex::new(T::one(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::one(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::one(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::one(), T::zero()),  
  ]
}

pub fn get_partial_swap<T: Float>(tau: T) -> [Complex<T>; 16]
{
  [
      Complex::new(tau.cos(), -tau.sin()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(tau.cos(), T::zero()),
      Complex::new(T::zero(), -tau.sin()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), -tau.sin()),
      Complex::new(tau.cos(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(T::zero(), T::zero()),
      Complex::new(tau.cos(), -tau.sin()),  
  ]
}
