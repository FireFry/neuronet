extern crate neuronet;
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let a = neuronet::Matrix::create(move |_, _| rng.gen::<f32>(), 3, 2);
    a.print_elements();
}
