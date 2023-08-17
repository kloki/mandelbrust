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
    #[arg(long, default_value_t = 500)]
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
    let data_set = calculate_data(
        args.x_min,
        args.x_max,
        args.y_max,
        args.y_min,
        args.iterations,
        args.width as usize,
        args.height as usize,
    );
    let colors = color_histogram(&data_set, args.iterations);
    graph(colors, args.width, args.height, args.output_file).unwrap()
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

fn calculate_data(
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

fn rgb_color(value: f64) -> [u8; 3] {
    if value > 0.9999 {
        return [0, 0, 0];
    } else if value > 0.72 {
        return [255, 255, 255];
    }
    [(120. * value) as u8, (120. * (1. - value)) as u8, 120]
}
fn color_histogram(data_set: &Vec<Vec<usize>>, max_iterations: usize) -> Vec<Vec<[u8; 3]>> {
    let mut iteration_counts = vec![0.0f64; max_iterations];
    let mut result = vec![vec![[0u8; 3]; data_set[0].len()]; data_set.len()];

    for row in data_set {
        for pixel_iteration in row {
            iteration_counts[pixel_iteration - 1] += 1.0;
        }
    }

    let total: f64 = iteration_counts.iter().sum();

    for (x, row) in data_set.iter().enumerate() {
        for (y, iteration) in row.iter().enumerate() {
            let mut value = 0.0;
            for i in 0..*iteration {
                value += iteration_counts[i] / total;
            }
            result[x][y] = rgb_color(value);
        }
    }

    result
}

fn graph(
    colors: Vec<Vec<[u8; 3]>>,
    width: u32,
    height: u32,
    filename: String,
) -> Result<(), ImageError> {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(colors[x as usize][y as usize]);
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
