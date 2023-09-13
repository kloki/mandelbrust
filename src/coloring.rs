use crate::mandlebrot::DataPoint;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Coloring {
    Blackwhite,
    Colors,
    Wiki,
}

const PALLETE: [[u8; 3]; 16] = [
    [66, 30, 15],
    [25, 7, 26],
    [9, 1, 47],
    [4, 4, 73],
    [0, 7, 100],
    [12, 44, 138],
    [24, 82, 177],
    [57, 125, 209],
    [134, 181, 229],
    [211, 236, 248],
    [241, 233, 191],
    [248, 201, 95],
    [255, 170, 0],
    [204, 128, 0],
    [153, 87, 0],
    [106, 52, 3],
];

fn colors(data_set: &Vec<Vec<DataPoint>>, max_iter: usize) -> Vec<Vec<[u8; 3]>> {
    data_set
        .iter()
        .map(|row| {
            row.iter()
                .map(|dp| {
                    if dp.iteration == max_iter {
                        return BLACK;
                    }
                    let scaling = 1.0 - dp.value.abs().log2().log10();
                    [
                        (255. * (1. - scaling)) as u8,
                        (200. * scaling) as u8,
                        (200. * scaling) as u8,
                    ]
                })
                .collect()
        })
        .collect()
}

fn black_white(data_set: &Vec<Vec<DataPoint>>, max_iter: usize) -> Vec<Vec<[u8; 3]>> {
    data_set
        .iter()
        .map(|row| {
            row.iter()
                .map(|dp| {
                    if dp.iteration == max_iter {
                        return BLACK;
                    }
                    WHITE
                })
                .collect()
        })
        .collect()
}

fn linear_interpolate(color_a: [u8; 3], color_b: [u8; 3], scaling: f64) -> [u8; 3] {
    let r = ((color_a[0] as f64 * 1. - scaling) + (color_b[0] as f64 * scaling)) as u8;
    let g = ((color_a[1] as f64 * 1. - scaling) + (color_b[1] as f64 * scaling)) as u8;
    let b = ((color_a[2] as f64 * 1. - scaling) + (color_b[2] as f64 * scaling)) as u8;
    [r, g, b]
}

fn wiki_histogram(data_set: &Vec<Vec<DataPoint>>, max_iter: usize) -> Vec<Vec<[u8; 3]>> {
    data_set
        .iter()
        .map(|row| {
            row.iter()
                .map(|dp| {
                    if dp.iteration == max_iter {
                        return BLACK;
                    }
                    let scaling = 1.0 - dp.value.abs().log2().log10();

                    linear_interpolate(
                        PALLETE[(adj_iteration as usize) % 16],
                        PALLETE[(adj_iteration as usize + 1) % 16],
                        adj_iteration % 1.,
                    )
                })
                .collect()
        })
        .collect()
}

pub fn get_colors(
    ct: Coloring,
    data_set: &Vec<Vec<DataPoint>>,
    max_iterations: usize,
) -> Vec<Vec<[u8; 3]>> {
    match ct {
        Coloring::Blackwhite => black_white(data_set, max_iterations),
        Coloring::Colors => colors(data_set, max_iterations),
        Coloring::Wiki => wiki_histogram(data_set, max_iterations),
    }
    // color_histogram(data_set, max_iterations)
}
