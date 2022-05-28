pub use tfix_derive::fixture;

pub trait TestFixture: Sized {
  fn set_up() -> Self;
  fn tear_down(self) {}
}

#[cfg(test)]
mod tests;
