use std::fs::File;
use std::io::Write;
use crate::mandelbrot::MandelbrotSet;

pub fn write_image_file(set: &MandelbrotSet, file_path: &str) -> std::io::Result<()>{
    let mut output_file = File::create(file_path)?;
    // Writing Magic Number to the File
    output_file.write("P2\n".as_bytes())?;

    // Writing Width and Height into the file
    output_file.write(&format!("{} {}\n", set.table_size, set.table_size).as_bytes())?;
    // write!(output_file, "{} {}\n", set.table_size, set.table_size)?;

    // Writing the maximum gray value
    output_file.write("255\n".as_bytes())?;

    // Write image data
    for c in 0..set.table_size{
        for r in 0..set.table_size{
            output_file.write(&format!("{} ", set.grid_table[r][c]).as_bytes())?;
        }
        output_file.write("\n".as_bytes())?;
    }

    Ok(())
}
