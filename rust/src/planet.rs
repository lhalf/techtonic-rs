use godot::engine::{MeshInstance3D, IMeshInstance3D};
use godot::log::godot_print;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=MeshInstance3D,tool)]
struct Planet {
    #[export]
    angular_speed: f64,
    base: Base<MeshInstance3D>
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
