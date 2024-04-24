use glam::{UVec2, Vec3};
use show_image::{create_window, ImageInfo, ImageView};

use crate::{double_buffer::DoubleBuffer, grid::Grid};

mod double_buffer;
mod grid;

fn luminance(rgb: Vec3) -> f32 {
    let [r, g, b] = rgb.to_array();
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn distance(lhs: UVec2, rhs: UVec2) -> f32 {
    if lhs == rhs {
        return 10.0;
    } // TEMP value until further considered

    let lhs = lhs.as_vec2();
    let rhs = rhs.as_vec2();
    lhs.distance(rhs) * CELLS_TO_DISTANCE
}

#[derive(Clone, Copy)]
struct Cell {
    color: Vec3,
    target: UVec2,
}

fn update_cell(pos: UVec2, reader: &Grid<Cell>) -> Cell {
    let mut target = reader[pos].target;
    let mut color = reader[pos].color;
    let mut nb_sum = Vec3::ZERO; // neighbour average
    let mut nb_count = 0.0;

    let better_target = |target, nb_target| {
        if luminance(reader[nb_target].color) / distance(pos, nb_target)
            > luminance(reader[target].color) / distance(pos, target)
        {
            nb_target
        } else {
            target
        }
    };

    if pos.x > 0 {
        let nb_cell = reader[pos - UVec2::new(1, 0)];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.color;
        nb_count += 1.0;
    }
    if pos.y > 0 {
        let nb_cell = reader[pos - UVec2::new(0, 1)];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.color;
        nb_count += 1.0;
    }
    if pos.x < WIDTH - 1 {
        let nb_cell = reader[pos + UVec2::new(1, 0)];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.color;
        nb_count += 1.0;
    }
    if pos.y < HEIGHT - 1 {
        let nb_cell = reader[pos + UVec2::new(0, 1)];
        target = better_target(target, nb_cell.target);
        nb_sum += nb_cell.color;
        nb_count += 1.0;
    }
    target = better_target(target, pos);

    let nb_avg = nb_sum / nb_count;
    color = color * (1.0 - ACC_FACTOR) + reader[target].color / distance(pos, target) * ACC_FACTOR;
    color = color * (1.0 - BLUR_FACTOR) + nb_avg * BLUR_FACTOR;

    Cell { color, target }
}

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;
const DEFAULT_COLOR: Vec3 = Vec3::ZERO;
const BLUR_FACTOR: f32 = 0.9;
const ACC_FACTOR: f32 = 0.7;
const CELLS_TO_DISTANCE: f32 = 32.0 / (HEIGHT as f32);

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut double_buf = DoubleBuffer::from_value(Grid::from_fn(WIDTH, HEIGHT, |x, y| Cell {
        color: DEFAULT_COLOR,
        target: UVec2::new(x, y),
    }));
    let window = create_window("image", Default::default())?;

    let lights = [
        (UVec2::new(40, 40), Vec3::new(1.0, 0.0, 0.0)),
        (UVec2::new(80, 70), Vec3::new(0.0, 0.0, 1.0)),
    ];

    loop {
        let (reader, writer) = double_buf.split();

        for pos in reader.iter_indices() {
            writer[pos] = update_cell(pos, reader);
        }

        for (pos, color) in lights {
            writer[pos].color = color;
        }
        double_buf.swap();

        let image_data: Vec<u8> = double_buf
            .reader()
            .iter()
            .flat_map(|v| v.color.to_array())
            .map(|x| (x.clamp(0.0, 1.0) * 256.0) as u8)
            .collect();

        window.set_image(
            "image-yep",
            ImageView::new(ImageInfo::rgb8(WIDTH, HEIGHT), &image_data),
        )?;
    }
}
