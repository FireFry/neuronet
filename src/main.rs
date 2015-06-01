extern crate neuronet;

use neuronet::Matrix;

fn main() {
    let x = Matrix::from_array(&[&[0f32, 0f32],
                                 &[0f32, 1f32],
                                 &[1f32, 0f32],
                                 &[1f32, 1f32]]);
    x.print_elements();
}
