extern crate neuronet;

fn main() {
    let a = neuronet::Matrix::<f32>::rand(3, 2);
    a.print_elements();
}
