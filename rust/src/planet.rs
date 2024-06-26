use godot::engine::{IMeshInstance3D, MeshInstance3D};
use godot::prelude::*;
use rand::Rng;

const CUBE_SIDE_COUNT: u64 = 6;

#[derive(GodotClass)]
#[class(base=MeshInstance3D,tool)]
struct Planet {
    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for Planet {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("{:?}", self.base().get_mesh());
    }
}

#[godot_api]
impl Planet {
    #[func]
    fn vertices(&self, length: u64) -> PackedVector3Array {
        use Direction::*;
        let mut vertices = PackedVector3Array::new();
        vertices.extend(generate_face(length as usize, XPositive));
        vertices.extend(generate_face(length as usize, XNegative));
        vertices.extend(generate_face(length as usize, YPositive));
        vertices.extend(generate_face(length as usize, YNegative));
        vertices.extend(generate_face(length as usize, ZPositive));
        vertices.extend(generate_face(length as usize, ZNegative));
        vertices
    }

    #[func]
    fn heights(&self, length: u64) -> PackedFloat32Array {
        let cells = length * length * CUBE_SIDE_COUNT;
        std::iter::from_fn(|| Some(rand::thread_rng().gen()))
            .take(cells as usize)
            .collect()
        // let increment = 0.95 / (cells) as f32;
        // (0..cells).map(|cell| cell as f32 * increment).collect()
    }
}

fn generate_face(length: usize, direction: Direction) -> impl Iterator<Item = Vector3> {
    (0..length).flat_map(move |row| {
        (0..length).flat_map(move |column| cell_verticies(length, direction, row, column))
    })
}

fn cell_verticies(
    length: usize,
    direction: Direction,
    row: usize,
    column: usize,
) -> impl Iterator<Item = Vector3> {
    let row_offset: f32 = (row as f32) / (length as f32);
    let column_offset: f32 = (column as f32) / (length as f32);
    let cell_size: f32 = 1.0 / (length as f32);
    [
        direction.vertex(row_offset, column_offset).normalized(),
        direction
            .vertex(row_offset + cell_size, column_offset)
            .normalized(),
        direction
            .vertex(row_offset, column_offset + cell_size)
            .normalized(),
        direction
            .vertex(row_offset + cell_size, column_offset)
            .normalized(),
        direction
            .vertex(row_offset + cell_size, column_offset + cell_size)
            .normalized(),
        direction
            .vertex(row_offset, column_offset + cell_size)
            .normalized(),
    ]
    .into_iter()
}

#[derive(Clone, Copy)]
enum Direction {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
    ZPositive,
    ZNegative,
}

impl Direction {
    fn vertex(&self, row_offset: f32, column_offset: f32) -> Vector3 {
        match self {
            Self::XPositive => Vector3 {
                x: 0.5,
                y: -0.5 + row_offset,
                z: 0.5 - column_offset,
            },
            Self::XNegative => Vector3 {
                x: -0.5,
                y: 0.5 - column_offset,
                z: -0.5 + row_offset,
            },
            Self::YPositive => Vector3 {
                x: -0.5 + row_offset,
                y: 0.5,
                z: -0.5 + column_offset,
            },
            Self::YNegative => Vector3 {
                x: -0.5 + column_offset,
                y: -0.5,
                z: -0.5 + row_offset,
            },
            Self::ZPositive => Vector3 {
                x: -0.5 + column_offset,
                y: -0.5 + row_offset,
                z: 0.5,
            },
            Self::ZNegative => Vector3 {
                x: -0.5 + row_offset,
                y: -0.5 + column_offset,
                z: -0.5,
            },
        }
    }
}
