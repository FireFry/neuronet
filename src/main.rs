extern crate neuronet;

fn main() {
    let a = neuronet::Matrix::create(|r, c| 10 * r + c, 3, 2);
    a.print_elements();
}
