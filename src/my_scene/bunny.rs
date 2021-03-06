#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial};
use raytracer::Octree;
use scene::{Camera, Scene};
use vec3::Vec3;

//// 300 polys, octree is slightly slower than no octree
//pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
//    Camera::new(
//        Vec3 { x: 0.0, y: -150.0, z: 30.0 },
//        Vec3 { x: 0.0, y: 60.0, z: 50.0 },
//        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
//        fov,
//        image_width,
//        image_height
//    )
//}
//
//pub fn get_scene() -> Scene {
//    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
//    lights.push(box SphereLight { position: Vec3 { x: 200.0, y: -200.0, z: 100.0 }, intensity: 1f64, radius: 40.0 });
//    lights.push(box SphereLight { position: Vec3 { x: -95.0, y: 20.0, z: 170.0 }, intensity: 0.5f64, radius: 15.0 });
//
//    let red   = CookTorranceMaterial { k_a: 0.1, k_d: 0.4, k_s: 0.5, k_sg: 0.5, k_tg: 0.0, gauss_constant: 5.0,  roughness: 0.05, ior: 0.98, ambient: 1.0, diffuse: 0.5, specular: 1.0, transmission: 0.0};
//    let green = CookTorranceMaterial { k_a: 0.0, k_d: 0.4, k_s: 0.6, k_sg: 0.7, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3,  ior: 1.5,  ambient: 1.0, diffuse: 0.5,  specular: 1.0, transmission: 0.0};
//    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 0.7, k_sg: 1.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.01, ior: 0.2,  ambient: 1.0, diffuse: 1.0,  specular: 1.0, transmission: 0.0};
//
//    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
//    prims.push(box Plane { a: 0.0, b: 0.0, c: 1.0, d: -10.0, material: box green});
//    prims.push(box Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 50.0 }, radius: 40.0, material: box shiny.clone() });
//    prims.push(box Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 140.0 }, radius: 40.0, material: box shiny.clone() });
//    let bunny = ::util::import::from_obj(red, false, "./docs/assets/models/bunny.obj");
//    for triangle in bunny.triangles.into_iter() { prims.push(triangle); }
//
//    println!("Generating octree...");
//    let octree = Octree::new_from_prims(prims);
//    println!("Octree generated...");
//
//    Scene {
//        lights: lights,
//        octree: octree,
//        background: 0.5,
//    }
//}
