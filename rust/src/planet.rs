use godot::engine::{IMeshInstance3D, MeshInstance3D};
use godot::log::godot_print;
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=MeshInstance3D,tool)]
struct Planet {
    #[export]
    angular_speed: f64,
    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for Planet {
    fn init(base: Base<MeshInstance3D>) -> Self {
        godot_print!("Hello, world!");

        Self {
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate_y(radians);
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
