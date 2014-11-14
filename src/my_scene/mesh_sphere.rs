#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial};
use mat4::{Mat4, Transform};
use raytracer::Octree;
use scene::{Camera, Scene};
use vec3::Vec3;

// When using Fresnel, set k_sg and k_tg (if applicable) to 1.0 for easier material definition.
// You can still manually tweak it if you wish (try reducing k_sg for metals)

// 2500 polys, marginal improvement from an octree
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 2.6, y: 2.0, z: -10.0 }, intensity: 1.0, radius: 1.0 });

    let porcelain = CookTorranceMaterial { k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, 
        gauss_constant: 5.0, roughness: 0.1, ior: 1.1, ambient: 1.0, 
        diffuse: 0.8, 
        specular: 1.0, 
        transmission: 0.0 
        };


    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    // prims.push(box Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box green });
    let sphere = ::util::import::from_obj(porcelain, false, "./Kugel_D=10mm.obj");
    
    for triangle in sphere.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: 0.2,
    }
}
