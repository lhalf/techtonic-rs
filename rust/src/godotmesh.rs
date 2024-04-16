use godot::prelude::*;

#[allow(dead_code)]
fn square() -> PackedVector3Array {
    let mut verticies = PackedVector3Array::new();
    verticies.push(Vector3::new(1.0, 1.0, 0.0));
    verticies.push(Vector3::new(0.0, 1.0, 0.0));
    verticies.push(Vector3::new(1.0, 0.0, 0.0));
    verticies.push(Vector3::new(0.0, 0.0, 0.0));
    verticies
}
