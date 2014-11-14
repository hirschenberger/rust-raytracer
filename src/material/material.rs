use vec3::Vec3;

/// TODO: Move specular/transmissive properties into traits
pub trait Material {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> f64;
    fn is_reflective(&self) -> bool;
    fn is_refractive(&self) -> bool;
    fn global_specular(&self, color: f64) -> f64;
    fn global_transmissive(&self, color: f64) -> f64;
    fn transmission(&self) -> f64;
    fn ior(&self) -> f64;
}
