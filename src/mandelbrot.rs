
const MAX_ITER: i32 = 1000;

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

    pub fn construct_set(&mut self, verbose: bool){
        let r_center = (self.table_size as f64/ 2.0).floor();
        let c_center = (self.table_size as f64/ 2.0).floor();
        let r_scale = r_center / 2.0;
        let c_scale = c_center / 2.0;
        for row in 0..self.table_size{
            if verbose {
                if row % 100 == 0{
                    println!("completing {:.3}%", (row as f64)/(self.table_size as f64) * 100.0);
                }
            }
            for col in 0..self.table_size{
                let c = ComplexNumber::new(((row as f64) - r_center) / r_scale, ((col as f64) - c_center) / c_scale);
                let is_set_member = self.check_mandelbrot_membership(&c);
                if is_set_member{
                    self.grid_table[row][col] = 255;
                }else{
                    self.grid_table[row][col] = 0;
                }
            }
        }
    }

    pub fn check_mandelbrot_membership(&self, c: &ComplexNumber) -> bool{
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
