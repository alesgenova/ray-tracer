use num_traits::float::{FloatCore as NumFloat};

pub trait Float : NumFloat {}

impl Float for f64 {}
impl Float for f32 {}
