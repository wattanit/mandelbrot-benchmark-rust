mod mandelbrot;
mod pgm;

use std::env;

fn main() {
    println!("Constructing mandelbrot set");

    let args: Vec<String> = env::args().collect();
    let table_size = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    }else{
        1001
    };

    let output_path = if args.len() > 2 {
        args[2].clone()
    }else{
        "output_img.pgm".to_string()
    };

    println!("Table size: {}", table_size);

    let mut set = mandelbrot::MandelbrotSet::new(table_size);
    set.construct_set(true);

    pgm::write_image_file(&set, &output_path).expect("Failed to write output file");
    println!("Wrote output file at {}", &output_path);

}
