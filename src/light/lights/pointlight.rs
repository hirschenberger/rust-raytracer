use light::light::Light;
use vec3::Vec3;

#[allow(dead_code)]
pub struct PointLight {
    pub position: Vec3,
    pub intensity: f64
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn intensity(&self) -> f64 {
        self.intensity
    }

    fn center(&self) -> Vec3 {
        self.position
    }

    fn is_point(&self) -> bool {
        true
    }
}
