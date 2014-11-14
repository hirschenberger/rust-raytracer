#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::Octree;
use scene::{Camera, Scene};
use vec3::Vec3;

// 50000 polys, model not included!
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -1500.0, y: 300.0, z: 600.0 },
        Vec3 { x: 0.0, y: 400.0, z: -200.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: -1400.0, y: 200.0, z: 100.0 }, intensity: 0.7, radius: 50.0 });

    let grey = CookTorranceMaterial { k_a: 0.0, k_d: 0.5, k_s: 0.8, k_sg: 0.5, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, ior: 0.4, ambient: 1.0, diffuse: 0.6, specular: 1.0, transmission: 0.0, diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let lucy = ::util::import::from_obj(grey, true, "./docs/assets/models/lucy.obj");
    for triangle in lucy.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: 0.5
    }
}
