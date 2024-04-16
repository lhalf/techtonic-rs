use godot::engine::{IMeshInstance3D, MeshInstance3D};
use godot::prelude::*;

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
}

#[godot_api]
impl Planet {
    #[func]
    fn square(&self) -> PackedVector3Array {
        let mut verticies = PackedVector3Array::new();
        verticies.push(Vector3::new(1.0, 0.0, 1.0));
        verticies.push(Vector3::new(1.0, 0.0, 0.0));
        verticies.push(Vector3::new(0.0, 0.0, 1.0));
        verticies.push(Vector3::new(1.0, 0.0, 0.0));
        verticies.push(Vector3::new(0.0, 0.0, 1.0));
        verticies.push(Vector3::new(0.0, 0.0, 0.0));
        verticies
    }
}
