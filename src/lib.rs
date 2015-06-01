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
        where F: FnMut(usize, usize) -> f32
    {
            
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

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols(), other.rows());
        Matrix::from_fn(|r, c| {
            let mut sum = 0f32;
            for i in 0..self.cols() {
                sum += self.get(r, i) * other.get(i, c);
            }
            sum
        }, self.rows(), other.cols())
    }

    pub fn map<F>(&self, mut f: F) -> Matrix
        where F: FnMut(f32) -> f32
    {
            
        Matrix::from_fn(move |r, c| f(*self.get(r, c)),
                        self.rows(),
                        self.cols())
    }
    
    pub fn sigmoid(&self) -> Matrix {
        self.map(|x| 1f32 / (1f32 + f32::exp(-x)))
    }

    pub fn minus(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows(), other.rows());
        assert_eq!(self.cols(), other.cols());
        Matrix::from_fn(|r, c| self.get(r, c) - other.get(r, c),
                        self.rows(),
                        self.cols())
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::from_fn(|r, c| *self.get(c, r), self.cols(), self.rows())
    }

    pub fn remove_bias(&self) -> Matrix {
        Matrix::from_fn(|r, c| *self.get(r, c + 1),
                        self.rows(),
                        self.cols() - 1)
    }

    pub fn product(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows(), other.rows());
        assert_eq!(self.cols(), other.cols());
        Matrix::from_fn(|r, c| self.get(r, c) * other.get(r, c),
                        self.rows(),
                        self.cols())
    }

    pub fn apply_polynomial(&self, a: &[f32]) -> Matrix {
        self.map(|x| {
            let mut sum = 0f32;
            let mut exp = 1f32;
            for coef in a {
                sum += coef * exp;
                exp *= x;
            }
            sum
        })
    }

    pub fn plus(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows(), other.rows());
        assert_eq!(self.cols(), other.cols());
        Matrix::from_fn(|r, c| self.get(r, c) + other.get(r, c),
                        self.rows(),
                        self.cols())
    }
}

