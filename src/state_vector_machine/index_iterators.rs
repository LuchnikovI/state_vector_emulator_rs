fn insert_zero(idx: usize, mask: usize) -> usize {
  ((mask & idx) << 1) | ((!mask) & idx)
}

pub(super) struct Q2IdexIterator {
  mask1: usize,
  mask2: usize,
  state: usize,
  end: usize,
}

impl Q2IdexIterator {
  pub(super) fn new(
    idx1: usize,
    idx2: usize,
    start: usize,
    len: usize,
  ) -> Self {
    let (min_idx, max_idx) = (std::cmp::min(idx1, idx2), std::cmp::max(idx1, idx2));
    Q2IdexIterator {
      mask1: usize::MAX << min_idx,
      mask2: usize::MAX << max_idx,
      state: start,
      end: len + start,
    }
  }
}

impl Iterator for Q2IdexIterator {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
      if self.state >= self.end {
        None
      } else {
        let curr_state = self.state;
        self.state += 1;
        Some(insert_zero(insert_zero(curr_state, self.mask1), self.mask2))
      }
  }
}

pub(super) struct Q1IdexIterator {
  mask: usize,
  state: usize,
  end: usize,
}

impl Q1IdexIterator {
  pub(super) fn new(
    idx: usize,
    start: usize,
    len: usize,
  ) -> Self {
    Q1IdexIterator {
      mask: usize::MAX << idx,
      state: start,
      end: len + start,
    }
  }
}

impl Iterator for Q1IdexIterator {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
      if self.state >= self.end {
        None
      } else {
        let curr_state = self.state;
        self.state += 1;
        Some(insert_zero(curr_state, self.mask))
      }
  }
}

/*
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn q1_index_iter_test() {
    let it = Q1IdexIterator::new(3, 0, 16);
    for i in it {
      println!("{:?}", i);
    }
  }
}
*/
