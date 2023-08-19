const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

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

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Coloring {
    Blackwhite,
    Histogram,
}
fn black_white(data_set: &Vec<Vec<usize>>, max_iteration: usize) -> Vec<Vec<[u8; 3]>> {
    data_set
        .iter()
        .map(|row| {
            row.iter()
                .map(|pixel| {
                    if *pixel == max_iteration {
                        return BLACK;
                    }
                    WHITE
                })
                .collect()
        })
        .collect()
}

pub fn get_colors(
    ct: Coloring,
    data_set: &Vec<Vec<usize>>,
    max_iterations: usize,
) -> Vec<Vec<[u8; 3]>> {
    match ct {
        Coloring::Blackwhite => black_white(data_set, max_iterations),
        Coloring::Histogram => color_histogram(data_set, max_iterations),
    }
    // color_histogram(data_set, max_iterations)
}
