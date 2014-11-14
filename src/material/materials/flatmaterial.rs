use material::Material;
use vec3::Vec3;

#[allow(dead_code)]
#[deriving(Clone)]
pub struct FlatMaterial {
    pub color: f64
}

impl Material for FlatMaterial {
    fn sample(&self, _n: Vec3, _i: Vec3, _l: Vec3, _u: f64, _v: f64) -> f64 {
        self.color
    }

    fn is_reflective(&self) -> bool {
        false
    }

    fn is_refractive(&self) -> bool {
        false
    }

    fn global_specular(&self, _color: f64) -> f64 {
        0.0
    }

    fn global_transmissive(&self, _color: f64) -> f64 {
        0.0
    }

    fn transmission(&self) -> f64 {
        0.0
    }

    fn ior(&self) -> f64 {
        1.0
    }
}
