extern crate rand;

use rand::{Rand, Rng};

pub struct Matrix {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn rows(&self) -> usize { self.rows }
    
    fn cols(&self) -> usize { self.cols }
    
    fn get(&self, r: usize, c: usize) -> &f32 {
        &self.data[Matrix::to_index(r, c, self.rows(), self.cols())]
    }
    
    fn to_coords(index: usize, rows: usize, cols: usize) -> (usize, usize) {
        (index / cols, index % cols)
    }

    fn to_index(r: usize, c: usize, rows: usize, cols: usize) -> usize {
        r * cols + c
    }
    
    pub fn print_elements(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}\t", self.get(r, c));
            }
            println!("");
        }
    }
    
    pub fn from_fn<F> (mut element_fn: F,
                      rows: usize,
                      cols: usize) -> Matrix
        where F: FnMut(usize, usize) -> f32 {
            
        let len = rows * cols;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let (r, c) = Matrix::to_coords(i, rows, cols);
            data.push(element_fn(r, c));
        }
        Matrix { data: data, rows: rows, cols: cols }
    }
    
    pub fn new(default_value: f32, rows: usize, cols: usize) -> Matrix {
        Matrix::from_fn(|_, _| default_value, rows, cols)
    }

    pub fn from_array(a: &[&[f32]]) -> Matrix {
        assert!(a.len() > 0);
        let rows = a.len();
        let cols = a[0].len();
        Matrix::from_fn(|r, c| a[r][c], rows, cols)
    }
    
    pub fn rand(rows: usize, cols: usize) -> Matrix {
        Matrix::from_fn(|_, _| rand::thread_rng().gen(), rows, cols)
    }
    
    pub fn add_bias(&self) -> Matrix {
        Matrix::from_fn(|r, c| {
            if c == 0 { 1f32 } else { *self.get(r, c - 1) }
        }, self.rows, self.cols + 1)
    }
}

