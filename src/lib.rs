extern crate rand;

use rand::{Rand, Rng};

pub trait Mat<T> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, r: usize, c: usize) -> &T;
}

pub struct Matrix<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl <T: Clone> Mat<T> for Matrix<T> {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn get(&self, r: usize, c: usize) -> &T {
        &self.data[Matrix::to_index(r, c, self.rows(), self.cols())]
    }
}

impl Matrix<()> {
    fn to_coords(index: usize, rows: usize, cols: usize) -> (usize, usize) {
        (index / cols, index % cols)
    }

    fn to_index(r: usize, c: usize, rows: usize, cols: usize) -> usize {
        r * cols + c
    }
}

impl <T: Clone + std::fmt::Display> Matrix<T> {
    pub fn print_elements(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}\t", self.get(r, c));
            }
            println!("");
        }
    }
}

impl <T: Clone + Rand> Matrix<T> {
    pub fn rand(rows: usize, cols: usize) -> Matrix<T> {
        Matrix::create(|_, _| rand::thread_rng().gen(), rows, cols)
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
