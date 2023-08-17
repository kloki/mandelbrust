use ::clap::Parser;
use image::ImageError;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = -2.0)]
    /// min x value
    x_min: f64,
    #[arg(long, default_value_t = 0.47)]
    /// max x value
    x_max: f64,
    #[arg(long, default_value_t = -1.12)]
    /// min y value
    y_min: f64,
    #[arg(long, default_value_t = 1.12)]
    /// max y value
    y_max: f64,
    #[arg(long, default_value_t = 1000)]
    /// Number of iterations per pixel
    iterations: usize,
    #[arg(short, long, default_value = "./output.png")]
    /// Name of png file
    output_file: String,
    #[arg(long, default_value_t = 800)]
    /// Width of png file in pixels.
    width: u32,
    /// Height of png file in pixels.
    #[arg(long, default_value_t = 800)]
    height: u32,
}
fn main() {
    let args = Args::parse();
    let data_set = calculate_set(
        args.x_min,
        args.x_max,
        args.y_min,
        args.y_max,
        args.iterations,
        args.width as usize,
        args.height as usize,
    );
    graph(data_set, args.width, args.height, args.output_file).unwrap()
}

fn run(x0: f64, y0: f64, max_iterations: usize) -> usize {
    let mut current_i = 0;
    let mut x2 = 0.;
    let mut y2 = 0.;
    let mut w = 0.;
    while (x2 + y2) <= 4. && current_i < max_iterations {
        let x = x2 - y2 + x0;
        let y = w - x2 - y2 + y0;
        x2 = x * x;
        y2 = y * y;
        w = (x + y) * (x + y);
        current_i += 1;
    }
    current_i
}

fn calculate_set(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iterations: usize,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let x_step = (x_max - x_min) / width as f64;
    let y_step = (y_max - y_min) / height as f64;
    let mut result = vec![vec![0; height]; width];

    for x in 0..width {
        for y in 0..height {
            result[x][y] = run(
                x_min + x as f64 * x_step,
                y_min + y as f64 * y_step,
                max_iterations,
            )
        }
    }

    result
}

fn graph(
    data_set: Vec<Vec<usize>>,
    width: u32,
    height: u32,
    filename: String,
) -> Result<(), ImageError> {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if data_set[x as usize][y as usize] == 1000 {
            *pixel = image::Rgb([0, 0, 0]);
        } else {
            *pixel = image::Rgb([255 as u8, 255, 255]);
        }
    }

    imgbuf.save(filename)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_main() {
        main()
    }
}
