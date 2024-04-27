use glam::{IVec2, UVec2, Vec3};
use rayon::iter::ParallelIterator;
use show_image::{create_window, ImageInfo, ImageView};

use double_buffer::DoubleBuffer;
use grid::Grid;

fn distance(lhs: UVec2, rhs: UVec2) -> f32 {
    let lhs = lhs.as_vec2();
    let rhs = rhs.as_vec2();
    lhs.distance(rhs) * CELLS_TO_DISTANCE
}

fn received_energy(reader: &Grid<Cell>, source: UVec2, sink: UVec2) -> f32 {
    if source == sink {
        return reader[source].energy;
    }

    let dir = (source.as_ivec2() - sink.as_ivec2()).as_vec2().normalize();
    let sign = (source.as_ivec2() - sink.as_ivec2()).signum();
    let x_offset = IVec2::new(sign.x, 0);
    let y_offset = IVec2::new(0, sign.y);

    let total = dir.x.abs() + dir.y.abs();

    let same_target = |p| (source == reader[p].target) as u32 as f32;

    let x_sample = (sink.as_ivec2() + x_offset).as_uvec2();
    let y_sample = (sink.as_ivec2() + y_offset).as_uvec2();

    let x_through = (dir.x.abs() / total) * same_target(x_sample);
    let y_through = (dir.y.abs() / total) * same_target(y_sample);

    let throughput = x_through + y_through;
    throughput * reader[source].energy / (distance(source, sink) + 1.0)
}

#[derive(Clone, Copy)]
struct Cell {
    energy: f32,
    target: UVec2,
}

fn update_cell(pos: UVec2, reader: &Grid<Cell>) -> Cell {
    let mut target = reader[pos].target;
    let mut energy = reader[pos].energy;
    let mut nb_sum = 0.0;
    let mut nb_count = 0.0;

    let better_target = |lhs: UVec2, rhs: UVec2| {
        if received_energy(reader, lhs, pos) >= received_energy(reader, rhs, pos) {
            lhs
        } else {
            rhs
        }
    };
    target = better_target(target, pos);

    if pos.x > 0 {
        let nb_cell = reader[pos - UVec2::X];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.energy;
        nb_count += 1.0;
    }
    if pos.y > 0 {
        let nb_cell = reader[pos - UVec2::Y];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.energy;
        nb_count += 1.0;
    }
    if pos.x < WIDTH - 1 {
        let nb_cell = reader[pos + UVec2::X];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.energy;
        nb_count += 1.0;
    }
    if pos.y < HEIGHT - 1 {
        let nb_cell = reader[pos + UVec2::Y];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.energy;
        nb_count += 1.0;
    }

    let nb_avg = nb_sum / nb_count;
    let recv_energy = received_energy(reader, target, pos);
    energy = energy * (1.0 - ACC_FACTOR) + recv_energy * ACC_FACTOR;
    energy = energy * (1.0 - BLUR_FACTOR) + nb_avg * BLUR_FACTOR;

    Cell { energy, target }
}

fn default_double_buffer() -> DoubleBuffer<[Grid<Cell>; 3]> {
    let default_grid = Grid::from_fn(WIDTH, HEIGHT, |x, y| Cell {
        energy: DEFAULT_ENERGY,
        target: UVec2::new(x, y),
    });
    DoubleBuffer::from_value([default_grid.clone(), default_grid.clone(), default_grid])
}

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;
const DEFAULT_ENERGY: f32 = 0.0;
const BLUR_FACTOR: f32 = 0.1;
const ACC_FACTOR: f32 = 1.0;
const CELLS_TO_DISTANCE: f32 = 32.0 / (HEIGHT as f32);

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut double_buf = default_double_buffer();
    let window = create_window("image", Default::default())?;

    let lights = [
        (UVec2::new(60, 45), Vec3::new(1.0, 0.0, 0.0)),
        (UVec2::new(80, 70), Vec3::new(0.0, 0.0, 1.0)),
        (UVec2::new(60, 90), Vec3::new(0.0, 1.0, 0.0)),
        (UVec2::new(10, 50), Vec3::new(0.5, 1.0, 0.0)),
        (UVec2::new(50, 10), Vec3::new(1.0, 0.0, 1.0)),
        (UVec2::new(60, 60), Vec3::new(0.2, 0.2, 0.2)),
    ];
    let mut barriers = vec![];

    for x in 36..50 {
        let y = 40 + x - 36;
        barriers.push(UVec2::new(x, y));
    }

    loop {
        let (reader, writer) = double_buf.split();

        for i in 0..3 {
            reader[i].iter_indices().for_each(|pos| {
                writer[i][pos] = update_cell(pos, &reader[i]);
            })
        }

        for (pos, color) in lights {
            writer[0][pos].energy = color.x;
            writer[1][pos].energy = color.y;
            writer[2][pos].energy = color.z;
        }
        for &pos in barriers.iter() {
            writer[0][pos] = Cell {
                energy: 0.0,
                target: pos,
            };
            writer[1][pos] = Cell {
                energy: 0.0,
                target: pos,
            };
            writer[2][pos] = Cell {
                energy: 0.0,
                target: pos,
            };
        }

        double_buf.swap();

        let image_data: Vec<u8> = double_buf.reader()[0]
            .iter_indices()
            .flat_map(|pos| {
                let reader = double_buf.reader();
                [
                    reader[0][pos].energy,
                    reader[1][pos].energy,
                    reader[2][pos].energy,
                ]
            })
            .map(|x| (x.clamp(0.0, 1.0) * 256.0) as u8)
            .collect();

        window.set_image(
            "image-yep",
            ImageView::new(ImageInfo::rgb8(WIDTH, HEIGHT), &image_data),
        )?;
    }
}
