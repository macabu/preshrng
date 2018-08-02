//! This lib implements a random number generator which outputs a unique usize-d integer.
//! Original version (in C++): https://github.com/preshing/RandomSequence
//! Article: http://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/

#![no_std]

fn permute_qpr(x: usize) -> usize {
  const PRIME: usize = 4_294_967_291;

  if x > PRIME {
    return x;
  }

  let residue = (x * x) % PRIME;

  if x <= PRIME / 2 {
    residue
  } else {
    PRIME - residue
  }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreshRng {
  seed: usize,
  index: usize,
  intermediate: usize,
}

impl PreshRng {
  #[inline(always)]
  pub fn default() -> Self {
    let seed = 0xdeadbeef as usize;

    let index = permute_qpr(permute_qpr(seed) + 0x682f0161);
    let intermediate = permute_qpr(permute_qpr(seed + 1) + 0x46790905);

    PreshRng {
      seed,
      index,
      intermediate,
    }
  }

  #[inline(always)]
  pub fn new(seed: usize) -> Self {
    let index = permute_qpr(permute_qpr(seed) + 0x682f0161);
    let intermediate = permute_qpr(permute_qpr(seed + 1) + 0x46790905);

    PreshRng {
      seed,
      index,
      intermediate,
    }
  }

  #[inline(always)]
  pub fn seed(&self) -> usize {
    self.seed
  }

  #[inline(always)]
  pub fn set_seed(&mut self, seed: usize) {
    self.seed = seed;
    self.index = permute_qpr(permute_qpr(seed) + 0x682f0161);
    self.intermediate = permute_qpr(permute_qpr(seed + 1) + 0x46790905);
  }

  #[inline(always)]
  pub fn next(&mut self) -> usize {
    self.index += 1;

    permute_qpr((permute_qpr(self.index) + self.intermediate) ^ 0x5bf03635)
  }
}
