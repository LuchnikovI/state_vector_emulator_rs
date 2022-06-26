#[macro_export]
macro_rules! Hadamard {
  ( $x:ty ) => { [
    Complex::new(1. / (2. * <$x>::one()).sqrt(), <$x>::zero()),
    Complex::new(1. / (2. * <$x>::one()).sqrt(), <$x>::zero()),
    Complex::new(1. / (2. * <$x>::one()).sqrt(), <$x>::zero()),
    Complex::new(-1. / (2. * <$x>::one()).sqrt(), <$x>::zero()),
  ]};
}

#[macro_export]
macro_rules! Phase {
  ( $x:ty, $phi:expr ) => { [
    Complex::new(<$x>::one()),
    Complex::new(<$x>::zero()),
    Complex::new(<$x>::zero()),
    Complex::new(($phi * <$x>::one()).cos(), ($phi * <$x>::one()).sin()),
  ]};
}

#[macro_export]
macro_rules! CX {
  ( $x:ty ) => { [
    Complex::new(<$x>::one(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::one(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::one(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
    Complex::new(<$x>::one(), <$x>::zero()),
    Complex::new(<$x>::zero(), <$x>::zero()),
  ]};
}

#[macro_export]
macro_rules! Heisenberg {
    ( $x:ty, $tau:expr) => {[
      Complex::new(($tau * <$x>::one()).cos(), -($tau * <$x>::one()).sin()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(($tau * <$x>::one()).cos(), <$x>::zero()),
      Complex::new(<$x>::zero(), -($tau * <$x>::one()).sin()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), -($tau * <$x>::one()).sin()),
      Complex::new(($tau * <$x>::one()).cos(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(<$x>::zero(), <$x>::zero()),
      Complex::new(($tau * <$x>::one()).cos(), -($tau * <$x>::one()).sin()),  
    ]};
}