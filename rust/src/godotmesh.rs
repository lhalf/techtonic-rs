use godot::engine::{IMeshInstance3D, MeshInstance3D};
use godot::log::godot_print;
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;

fn square() -> PackedVector3Array {
    let mut verticies = PackedVector3Array::new();
    verticies.push(Vector3::new(1.0, 1.0, 0.0));
    verticies.push(Vector3::new(0.0, 1.0, 0.0));
    verticies.push(Vector3::new(1.0, 0.0, 0.0));
    verticies.push(Vector3::new(0.0, 0.0, 0.0));
    verticies
}
