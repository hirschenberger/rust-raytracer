use material::{Material, Texture};
use vec3::Vec3;

#[allow(dead_code)]
#[deriving(Clone)]
pub struct PhongMaterial {
    pub k_a: f64,           // Ambient coefficient
    pub k_d: f64,           // Diffuse coefficient
    pub k_s: f64,           // Local specular coefficient
    pub k_sg: f64,          // Global specular coefficient (mirror reflection)
    pub k_tg: f64,          // Global transmissive coefficient (refraction)
    pub ambient: f64,      // Ambient color
    pub diffuse: f64,      // Diffuse color
    pub transmission: f64, // Transmissive color
    pub specular: f64,     // Specular color
    pub shininess: f64,     // Size of Phong specular highlight
    pub ior: f64,           // Index of refraction
    pub diffuse_texture: Option<Box<Texture+Send+Sync>>
}

impl Material for PhongMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> f64 {
        let h = (l + i).unit();

        // Blinn-Phong approximation
        let ambient  = self.ambient * self.k_a;
        let diffuse  = self.diffuse * self.k_d * n.dot(&l);
        let specular = (self.specular * self.k_s * n.dot(&h)).powf(self.shininess);

        ambient + diffuse + specular
    }

    fn is_reflective(&self) -> bool {
        self.k_sg > 0.0
    }

    fn is_refractive(&self) -> bool {
        self.k_tg > 0.0
    }

    fn global_specular(&self, color: f64) -> f64 {
        color * self.k_sg
    }

    fn global_transmissive(&self, color: f64) -> f64 {
        color * self.k_tg
    }

    fn transmission(&self) -> f64 {
        self.transmission
    }

    fn ior(&self) -> f64 {
        self.ior
    }
}
