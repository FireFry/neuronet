extern crate rand;

pub struct Matrix<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl Matrix<()> {
    fn to_coords(index: usize, rows: usize, cols: usize) -> (usize, usize) {
        (index / cols, index % cols)
    }
}

impl <T: Clone + std::fmt::Display> Matrix<T> {
    pub fn print_elements(&self) {
        for x in self.data.iter() {
            println!("{}", x);
        }
    }
}

impl <T: Clone> Matrix<T> {
    pub fn create<F> (mut element_fn: F,
                      rows: usize,
                      cols: usize) -> Matrix<T>
        where F: FnMut(usize, usize) -> T {
            
        let len = rows * cols;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let (r, c) = Matrix::to_coords(i, rows, cols);
            data.push(element_fn(r, c));
        }
        Matrix { data: data, rows: rows, cols: cols }
    }
    
    pub fn new(default_value: T, rows: usize, cols: usize) -> Matrix<T> {
        Matrix::create(|_, _| default_value.clone(), rows, cols)
    }
}
