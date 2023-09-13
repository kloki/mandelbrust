use ::clap::Parser;
use ::std::{
    io,
    io::Write,
    time::Instant,
};
use coloring::{
    get_colors,
    Coloring,
};
use image::ImageError;
use mandlebrot::calculate_data;
mod coloring;
mod mandlebrot;

use console::{
    style,
    Emoji,
    Style,
};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// x coordinate center
    #[arg(short,long, default_value_t = -0.5)]
    x: f64,
    /// y coordinate center
    #[arg(short, long, default_value_t = 0.0)]
    y: f64,
    /// zoom
    #[arg(long, default_value_t = 0.8)]
    zoom: f64,
    /// Number of iterations per pixel
    #[arg(long, default_value_t = 500)]
    iterations: usize,
    /// Name of png file
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
    /// Width of png file in pixels.
    #[arg(long, default_value_t = 1000)]
    width: u32,
    /// Height of png file in pixels.
    #[arg(long, default_value_t = 1000)]
    height: u32,
    /// Coloring scheme used.
    #[arg(value_enum, long, default_value_t = Coloring::Colors)]
    coloring: Coloring,
}

static CALCULATING: Emoji<'_, '_> = Emoji("🧮 ", "");
static COLORING: Emoji<'_, '_> = Emoji("🌈 ", "");
static PNG: Emoji<'_, '_> = Emoji("🖼️  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn get_window(x: f64, y: f64, zoom: f64) -> (f64, f64, f64, f64) {
    let window = 1. / zoom;
    (x - window, x + window, y - window, y + window)
}

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
    let (x_min, x_max, y_min, y_max) = get_window(args.x, args.y, args.zoom);
    let data_set = calculate_data(
        x_min,
        x_max,
        y_max,
        y_min,
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
