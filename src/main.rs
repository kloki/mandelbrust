use ::clap::Parser;
use image::ImageError;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    graph(args.width, args.height, args.output_file).unwrap()
}

fn graph(width: u32, height: u32, filename: String) -> Result<(), ImageError> {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save(filename)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::graph;

    #[test]
    fn test_basic() {
        graph(50, 50, "test.png".to_string()).unwrap();
    }
}
