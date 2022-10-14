use std::collections::VecDeque;

use piet::kurbo::Affine;
#[repr(C)]
pub struct Primitive {
    transform: [[f64; 4]; 4],
    color: [f64; 4],
}

pub struct Cache {
    primitives: VecDeque<Vec<Primitive>>,
}
