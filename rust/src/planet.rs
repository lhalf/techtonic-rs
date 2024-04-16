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

    fn ready(&mut self) {
        godot_print!("{:?}", self.base().get_mesh());
    }
}

#[godot_api]
impl Planet {
    #[func]
    fn vertices(&self) -> PackedVector3Array {
        let mut vertices = PackedVector3Array::new();
        vertices.push(Vector3::new(0.0, 0.0, 0.0));
        vertices.push(Vector3::new(1.0, 0.0, 0.0));
        vertices.push(Vector3::new(0.0, 0.0, 1.0));
        vertices.push(Vector3::new(1.0, 0.0, 0.0));
        vertices.push(Vector3::new(1.0, 0.0, 1.0));
        vertices.push(Vector3::new(0.0, 0.0, 1.0));
        vertices
    }
}

