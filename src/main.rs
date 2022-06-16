use ndarray::{Array, Array2};

fn main() {
    println!("Hello, world!");
}

pub fn _poc(radius: usize) -> Array2<f32> {
    let dim = radius * 2 + 1;
    let mut filter = Array::zeros((dim, dim));
    for y in 0..dim {
        for x in 0..dim {
            let l = (y as f32 * y as f32 + x as f32 * x as f32).sqrt();
            filter[(y, x)] = l;
        }
    }
    filter
}