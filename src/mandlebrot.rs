use rayon::prelude::*;
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

pub fn calculate_data(
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
    (0..width)
        .into_par_iter()
        .map(|x| {
            (0..height)
                .into_iter()
                .map(|y| {
                    run(
                        x_min + x as f64 * x_step,
                        y_min + y as f64 * y_step,
                        max_iterations,
                    )
                })
                .collect::<Vec<usize>>()
        })
        .collect()
}
