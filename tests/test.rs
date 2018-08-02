extern crate preshrng;

#[cfg(test)]
mod tests {
  use preshrng::PreshRng;
  use std::time::{SystemTime, UNIX_EPOCH};

  #[test]
  fn default_presh() {
    let mut default_presh = PreshRng::default();

    assert_eq!(0xdeadbeef, default_presh.seed());
    assert_eq!(6161166187, default_presh.next());
    assert_eq!(6857426436, default_presh.next());
    assert_eq!(4466716990, default_presh.next());

    default_presh.set_seed(0xdeadbeef as usize);

    assert_eq!(0xdeadbeef, default_presh.seed());
    assert_eq!(6161166187, default_presh.next());
    assert_eq!(6857426436, default_presh.next());
    assert_eq!(4466716990, default_presh.next());
  }

  #[test]
  fn new_presh_default_seed() {
    let seed = 0xdeadbeef as usize;

    let mut new_presh = PreshRng::new(seed);

    assert_eq!(0xdeadbeef, new_presh.seed());
    assert_eq!(6161166187, new_presh.next());
    assert_eq!(6857426436, new_presh.next());
    assert_eq!(4466716990, new_presh.next());
  }

  #[test]
  fn new_presh() {
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs() as usize;

    let mut new_presh = PreshRng::new(seed);

    assert_eq!(seed, new_presh.seed());
    assert_ne!(new_presh.next(), new_presh.next());
  }
}
