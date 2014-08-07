#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::{Octree, VecPrimContainer};
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// 5000 polys, cow. Octree helps.
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -2.0, y: 4.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0 });

    let red   = CookTorranceMaterial { k_a: 0.0, k_d: 0.6, k_s: 1.0, k_sg: 0.2, k_tg: 0.0, gauss_constant: 30.0, roughness: 0.1, ior: 0.8, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 0.25, z: 0.1 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let green = CookTorranceMaterial { k_a: 0.0, k_d: 0.5, k_s: 0.4, k_sg: 0.1, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.4, ior: 0.95, ambient: Vec3::one(), diffuse: Vec3 { x: 0.2, y: 0.7, z: 0.2 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane { a: 0.0, b: 1.0, c: 0.0, d: 3.6, material: box green });
    let cow = ::util::import::from_obj(Vec3::zero(), 1.0, red, true, "./docs/assets/models/cow.obj");
    for triangle in cow.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        skybox: None
    }
}