use super::*;
use tfix_derive::fixture;

#[derive(Default)]
struct Test {
    it_works: bool,
}

impl Test {
    fn test(&mut self) {
        self.it_works = true;
    }
}

impl TestFixture for Test {
    fn set_up() -> Self {
        Self::default()
    }

    fn tear_down(self) {
        assert!(self.it_works);
    }
}

#[fixture(Test)]
mod test_fixture {
    use super::*;
    fn run(t: &mut Test) {
        t.test();
    }
}
