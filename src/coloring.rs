use crate::mandlebrot::DataPoint;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Coloring {
    Blackwhite,
    Colors,
    Wiki,
}
fn pallete(scaling: f64) -> [u8; 3] {
    [(255. * (1. - scaling)) as u8, 200, (200. * scaling) as u8]
}

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
                    pallete(scaling)
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

fn wiki_histogram(data_set: &Vec<Vec<DataPoint>>, max_iter: usize) -> Vec<Vec<[u8; 3]>> {
    data_set
        .iter()
        .map(|row| {
            row.iter()
                .map(|dp| {
                    if dp.iteration == max_iter {
                        return BLACK;
                    }
                    match dp.iteration % 16 {
                        0 => [66, 30, 15],
                        1 => [25, 7, 26],
                        2 => [9, 1, 47],
                        3 => [4, 4, 73],
                        4 => [0, 7, 100],
                        5 => [12, 44, 138],
                        6 => [24, 82, 177],
                        7 => [57, 125, 209],
                        8 => [134, 181, 229],
                        9 => [211, 236, 248],
                        10 => [241, 233, 191],
                        11 => [248, 201, 95],
                        12 => [255, 170, 0],
                        13 => [204, 128, 0],
                        14 => [153, 87, 0],
                        _ => [106, 52, 3],
                    }
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
