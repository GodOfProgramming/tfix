use super::*;
use tfix_derive::fixture;

#[derive(Default)]
struct Test {
  counter: u8,
}

impl Test {
  fn new(counter: u8) -> Self {
    Self { counter }
  }

  fn test(&mut self) {
    self.counter += 1;
  }

  #[allow(unused)]
  fn fail(&mut self) {
    panic!("intentional fail");
  }
}

impl TestFixture for Test {
  fn set_up() -> Self {
    Self::new(1)
  }

  fn tear_down(mut self) {
    self.counter += 1;
  }
}

impl Drop for Test {
  fn drop(&mut self) {
    assert_eq!(self.counter, 3);
  }
}

#[fixture(Test)]
mod test_fixture {
  use super::*;

  #[test]
  fn should_be_tested(t: &mut Test) {
    t.test();
  }

  #[allow(unused)]
  fn should_not_be_tested(t: &mut Test) {
    t.fail();
  }
}
