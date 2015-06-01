extern crate neuronet;

use neuronet::Matrix;

fn main() {
    let x = Matrix::from_array(&[&[0f32, 0f32],
                                 &[0f32, 1f32],
                                 &[1f32, 0f32],
                                 &[1f32, 1f32]]);

    let y = Matrix::from_array(&[&[0f32],
                                 &[1f32],
                                 &[1f32],
                                 &[0f32]]);

    let mut w0 = Matrix::rand(3, 2);
    let mut w1 = Matrix::rand(3, 1);        

    let learning_rate = 1f32;
    let iterations = 10000;
    
    for i in 0..iterations {
        let a1 = x.add_bias().multiply(&w0).sigmoid();
        let a2 = a1.add_bias().multiply(&w1).sigmoid();

        let d2 = y.minus(&a2);
        let d1 = d2.multiply(&w1.transpose())
            .remove_bias()
            .product(&a1.apply_polynomial(&[0f32, 1f32, -1f32]));
        
        w1 = a1
            .add_bias()
            .transpose()
            .multiply(&d2)
            .apply_polynomial(&[0f32, learning_rate])
            .plus(&w1);

        w0 = x
            .add_bias()
            .transpose()
            .multiply(&d1)
            .apply_polynomial(&[0f32, learning_rate])
            .plus(&w0);
    }

    x.add_bias()
        .multiply(&w0)
        .sigmoid()
        .add_bias()
        .multiply(&w1)
        .sigmoid()
        .print_elements();
}
