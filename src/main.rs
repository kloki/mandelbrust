use ::clap::Parser;
use ::std::io;
use ::std::io::Write;
use ::std::time::Instant;
use coloring::{get_colors, Coloring};
use image::ImageError;
use mandlebrot::calculate_data;
mod coloring;
mod mandlebrot;

use console::{style, Emoji, Style};
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
    #[arg(long, default_value_t = 1000)]
    /// Width of png file in pixels.
    width: u32,
    /// Height of png file in pixels.
    #[arg(long, default_value_t = 900)]
    height: u32,
    /// Coloring scheme used.
    #[arg(long, default_value_t = Coloring::Blackwhite)]
    #[arg(value_enum)]
    coloring: Coloring,
}

static CALCULATING: Emoji<'_, '_> = Emoji("🧮 ", "");
static COLORING: Emoji<'_, '_> = Emoji("🌈 ", "");
static PNG: Emoji<'_, '_> = Emoji("🖼️  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    let green = Style::new().green();
    let args = Args::parse();

    let instant = Instant::now();
    print!(
        "{} {}Calculating data...       ",
        style("[1/3]").bold().dim(),
        CALCULATING
    );
    io::stdout().flush().unwrap();
    let data_set = calculate_data(
        args.x_min,
        args.x_max,
        args.y_max,
        args.y_min,
        args.iterations,
        args.width as usize,
        args.height as usize,
    );
    println!("{:?}", green.apply_to(instant.elapsed()));

    let instant = Instant::now();
    print!(
        "{} {}Calculating Colors...     ",
        style("[2/3]").bold().dim(),
        COLORING
    );
    io::stdout().flush().unwrap();
    let colors = get_colors(args.coloring, &data_set, args.iterations);
    println!("{:?}", green.apply_to(instant.elapsed()));

    let instant = Instant::now();
    print!(
        "{} {}Creating PNG...           ",
        style("[3/3]").bold().dim(),
        PNG
    );
    io::stdout().flush().unwrap();
    graph(colors, args.width, args.height, args.output_file).unwrap();
    println!("{:?}", green.apply_to(instant.elapsed()));

    println!("{} Done", SPARKLE);
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
