use vec3::Vec3;

pub trait Light {
    fn position(&self) -> Vec3;
    fn intensity(&self) -> f64;
    fn center(&self) -> Vec3;
    fn is_point(&self) -> bool;
}
