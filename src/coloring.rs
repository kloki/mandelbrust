use crate::mandlebrot::DataPoint;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

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

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Coloring {
    Blackwhite,
    Colors,
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

pub fn get_colors(
    ct: Coloring,
    data_set: &Vec<Vec<DataPoint>>,
    max_iterations: usize,
) -> Vec<Vec<[u8; 3]>> {
    match ct {
        Coloring::Blackwhite => black_white(data_set, max_iterations),
        Coloring::Colors => colors(data_set, max_iterations),
    }
    // color_histogram(data_set, max_iterations)
}
