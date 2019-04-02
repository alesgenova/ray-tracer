use num_traits::float::{FloatCore as NumFloat};

pub trait Number {
  fn sqrt(&self) -> Self;
  fn tan(&self) -> Self;
}

impl Number for f64 {
  fn sqrt(&self) -> Self {
    f64::sqrt(*self)
  }

  fn tan(&self) -> Self {
    f64::tan(*self)
  }
}
impl Number for f32 {
  fn sqrt(&self) -> Self {
    f32::sqrt(*self)
  }

  fn tan(&self) -> Self {
    f32::tan(*self)
  }
}

pub trait Float : 'static + NumFloat + Number {}

impl Float for f64 {}
impl Float for f32 {}
