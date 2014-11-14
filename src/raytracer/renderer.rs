use std::rand::{task_rng, Rng, SeedableRng, Isaac64Rng};
use std::sync::Arc;
use std::sync::deque::{BufferPool, Data, Empty, Abort};
use raytracer::compositor::{ColorRGBA, Surface, SurfaceFactory};
use raytracer::{Intersection, Ray};
use light::Light;
use scene::{Camera, Scene};
use vec3::Vec3;

pub static EPSILON: f64 = ::std::f64::EPSILON * 10000.0;

pub struct Renderer {
    pub shadow_samples: uint, // Number of samples for soft shadows and area lights.
    pub pixel_samples: uint,  // The square of this is the number of samples per pixel.
    pub tasks: uint           // Minimum number of tasks to spawn.
}

impl Renderer {
    pub fn render(&self, camera: Camera, shared_scene: Arc<Scene>) -> Surface {

        let mut surface = Surface::new(camera.image_width as uint,
                                       camera.image_height as uint,
                                       ColorRGBA::new_rgb(0, 0, 0));

        let (worker, stealer) = BufferPool::new().deque();
        let (tx, rx) = channel();

        let mut jobs = 0;
        for subsurface_factory in surface.divide(128, 8) {
            jobs += 1;
            worker.push(subsurface_factory);
        }

        for _ in range(0, self.tasks) {
            let renderer = *self.clone();
            let child_tx = tx.clone();
            let child_stealer = stealer.clone();
            let scene_local = shared_scene.clone();
            let camera_local = camera.clone();

            spawn(proc() {
                loop {
                    match child_stealer.steal() {
                        Data(factory) => {
                            child_tx.send(renderer.render_tile(camera_local.clone(),
                                                               scene_local.deref(),
                                                               factory))
                        },
                        Empty => break,
                        Abort => ()
                    }
                }
            });
        }

        let start_time = ::time::get_time();

        for i in range(0, jobs) {
            surface.merge(rx.recv());
            ::util::print_progress("Tile", start_time, (i + 1) as uint, jobs);
        }

        surface
    }

    fn render_tile(&self, camera: Camera, scene: &Scene,
                   tile_factory: SurfaceFactory) -> Box<Surface> {

        let shadow_samples = self.shadow_samples;
        let pixel_samples = self.pixel_samples;

        let mut tile = tile_factory.create();

        let mut random_data = [0u64, ..64];
        for i in range(0u, 64u) {
            random_data[i] = task_rng().next_u64();
        }
        let mut rng: Isaac64Rng = SeedableRng::from_seed(random_data.clone());

        for rel_y in range(0u, tile.height) {
            let abs_y = (camera.image_height as uint) - (tile.y_off + rel_y) - 1;
            for rel_x in range(0u, tile.width) {
                let abs_x = tile.x_off + rel_x;

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / pixel_samples as f64;
                let mut color = Vec3::zero();

                for y_subpixel in range(0, pixel_samples) {
                    for x_subpixel in range(0, pixel_samples) {
                        // Don't jitter if not antialiasing
                        let (j_x, j_y) = if pixel_samples > 1 {
                            (x_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width,
                             y_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width)
                        } else {
                            (0.0, 0.0)
                        };

                        let ray = camera.get_ray(abs_x as f64 + j_x, abs_y as f64 + j_y);
                        let result = Renderer::trace(scene, &ray, shadow_samples);
                        // Clamp subpixels for now to avoid intense aliasing when combined value is clamped later
                        // Should think of a better way to handle this
                        color = color + result.clamp(0.0, 1.0).scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }
                tile[(rel_x, rel_y)] = ColorRGBA::new_rgb_clamped(color.x, color.y, color.z);
            }
        }

        box tile
    }

    fn trace(scene: &Scene, ray: &Ray, shadow_samples: uint) -> Vec3 {
        match ray.get_nearest_hit(scene) {
            Some(hit) => {
                let n = hit.n.unit();
                let i = (-ray.direction).unit();

                // Local lighting computation: surface shading, shadows
                let result = scene.lights.iter().fold(Vec3::zero(), |color_acc, light| {
                    let shadow = Renderer::shadow_intensity(scene, &hit, light, shadow_samples);
                    let l = (light.center() - hit.position).unit();

                    color_acc + light.color() * hit.material.sample(n, i, l, hit.u, hit.v) * shadow
                });
                result
            },
            None => { scene.background }
        }
    }

    fn shadow_intensity(scene: &Scene, hit: &Intersection,
                        light: &Box<Light+Send+Sync>, shadow_samples: uint) -> Vec3 {

        if shadow_samples <= 0 { return Vec3::one() }

        // Point light speedup (no point in sampling a point light multiple times)
        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };
        let mut shadow = Vec3::zero();

        // Take average shadow color after jittering/sampling light position
        for _ in range(0, shadow_sample_tries) {
            // L has to be a unit vector for t_max 1:1 correspondence to
            // distance to light to work. Shadow feelers only search up
            // until light source.
            let sampled_light_position = light.position();
            let shadow_l = (sampled_light_position - hit.position).unit();
            let shadow_ray = Ray::new(hit.position, shadow_l);
            let distance_to_light = (sampled_light_position - hit.position).len();

            // Check against candidate primitives in scene for occlusion
            // and multiply shadow color by occluders' shadow colors
            let mut candidate_nodes = scene.octree.get_intersected_objects(&shadow_ray);

            shadow = shadow + candidate_nodes.fold(Vec3::one(), |shadow_acc, prim| {
                let occlusion = prim.intersects(&shadow_ray, EPSILON, distance_to_light);
                match occlusion {
                    Some(occlusion) => shadow_acc * occlusion.material.transmission(),
                    None => shadow_acc
                }
            });
        }

        shadow.scale(1.0 / shadow_sample_tries as f64)
    }

}
