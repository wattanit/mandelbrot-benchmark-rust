
const MAX_ITER: i32 = 1000;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub struct MandelbrotSet{
    pub grid_table: Vec<Vec<i32>>,
    pub table_size: usize,
}

pub struct ComplexNumber{
    re: f64,
    im: f64,
}

impl ComplexNumber{
    pub fn new(re: f64, im: f64) -> ComplexNumber{
        ComplexNumber{re, im}
    }

    pub fn square(&self) -> ComplexNumber{
        ComplexNumber::new(self.re * self.re - self.im * self.im, 2.0 * self.re * self.im)
    }

    pub fn add(&self, other: &ComplexNumber) -> ComplexNumber{
        ComplexNumber::new(self.re + other.re, self.im + other.im)
    }

    pub fn multiply(&self, other: &ComplexNumber) -> ComplexNumber{
        ComplexNumber::new(self.re * other.re - self.im * other.im, self.re * other.im + self.im * other.re)
    }

    pub fn abs(&self) -> f64{
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl MandelbrotSet{
    pub fn new(table_size: usize) -> MandelbrotSet{
        let mut grid_table = Vec::new();
        for _ in 0..table_size{
            let mut row = Vec::new();
            for _ in 0..table_size{
                row.push(0);
            }
            grid_table.push(row);
        }
        MandelbrotSet{grid_table, table_size}
    }

    pub fn construct_set(&mut self, parallel: bool, verbose: bool){
        if parallel {
            self.construct_set_parallel(verbose);
        }else{
            self.construct_set_sequential(verbose);
        }
    }

    fn construct_set_sequential(&mut self, verbose: bool){
        let r_center = (self.table_size as f64/ 2.0).floor();
        let c_center = (self.table_size as f64/ 2.0).floor();
        let r_scale = r_center / 2.0;
        let c_scale = c_center / 2.0;

        for row in 0..self.table_size{
            if verbose {
                if row % 100 == 0{
                    println!("completing {:.3}%", ((row+1) as f64)/(self.table_size as f64) * 100.0);
                }
            }

            for col in 0..self.table_size{
                let c = ComplexNumber::new(((row as f64) - r_center) / r_scale, ((col as f64) - c_center) / c_scale);
                let is_set_member = MandelbrotSet::check_mandelbrot_membership(&c);
                if is_set_member{
                    self.grid_table[row][col] = 255;
                }else{
                    self.grid_table[row][col] = 0;
                }
            }
        }
    }

    fn construct_set_parallel(&mut self, verbose: bool){
        let r_center = (self.table_size as f64/ 2.0).floor();
        let c_center = (self.table_size as f64/ 2.0).floor();
        let r_scale = r_center / 2.0;
        let c_scale = c_center / 2.0;
        let table_size = self.table_size;

        let (tx, rx) = mpsc::channel();

        for row in 0..table_size{
            let local_tx = tx.clone();
            let mut row_vec = vec![0; table_size];
            thread::spawn(move || {
                for col in 0..table_size {
                    let c = ComplexNumber::new(((row as f64) - r_center) / r_scale, ((col as f64) - c_center) / c_scale);
                    let is_set_member = MandelbrotSet::check_mandelbrot_membership(&c);
                    if is_set_member {
                        row_vec[col] = 255;
                    } else {
                        row_vec[col] = 0;
                    };
                }
                local_tx.send((row, row_vec)).unwrap();
            });
        }
        let mut received_row_count = 0;
        for received in &rx{
            received_row_count += 1;
            self.grid_table[received.0] = received.1;
            if verbose {
                if received_row_count % 100 == 0{
                    println!("completing {:.3}%", ((received_row_count+1) as f64)/(self.table_size as f64) * 100.0);
                }
            }
            if received_row_count >= self.table_size{
                break;
            }
        }
    }

    pub fn check_mandelbrot_membership(c: &ComplexNumber) -> bool{
        let mut z = ComplexNumber::new(0.0, 0.0);
        for _ in 0..MAX_ITER{
            let z_squared = z.square();
            let z_new = z_squared.add(c);
            if z_new.abs() >= 2.0{
                return false;
            }else{
                z = z_new;
            }
        }
        return true;
    }
}
