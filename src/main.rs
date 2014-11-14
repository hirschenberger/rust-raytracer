#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(tuple_indexing)]

extern crate time;
extern crate serialize;

use scene::{Camera, Scene};

use std::io::File;
use std::io;
use std::os;
use std::sync::Arc;
use serialize::json;
use serialize::json::MissingFieldError;

mod geometry;
mod light;
mod material;
mod my_scene;
mod raytracer;
mod scene;
mod util;
mod vec3;
mod mat4;

// Replace this with argparse eventually
struct ProgramArgs {
    config_file: String
}

#[deriving(Decodable, Encodable)]
struct SceneConfig<'a> {
    size: (int, int),
    fov: f64,
    focal_dist: (f64, f64),
    min_intensity: u8,
    shadow_samples: uint,
    pixel_samples: uint,
    output_file: String
}

fn parse_args(args: Vec<String>) -> Result<ProgramArgs, String>  {
    let (program_name, rest) = match args[] {
        // I wouldn't expect this in the wild
        [] => panic!("Args do not even include a program name"),
        [ref program_name, rest..] => (
            program_name,
            rest
        )
    };
    match rest.len() {
        0 => Err(format!("Usage: {} scene_config.json", program_name)),
        1 => Ok(ProgramArgs {
            config_file: rest[0].clone()
        }),
        _ => Err(format!("Usage: {} scene_config.json", program_name)),
    }
}


fn main() {
    let start_time = ::time::get_time().sec;

    let program_args = match parse_args(os::args()) {
        Ok(program_args) => program_args,
        Err(mut error_str) => {
            error_str.push_str("\n");
            let mut stderr = io::stderr();
            assert!(stderr.write(error_str.as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let config_path = Path::new(program_args.config_file);
    let mut file_handle = match File::open(&config_path) {
        Ok(file) => file,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}\n", err).as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let json_data = match file_handle.read_to_string() {
        Ok(data) => data,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}\n", err).as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    let config: SceneConfig = match json::decode(json_data[]) {
        Ok(data) => data,
        Err(err) => {
            let mut stderr = io::stderr();
            let msg = match err {
                MissingFieldError(field_name) => {
                    format!("parse failure, missing field ``{}''\n", field_name)
                },
                _ => {
                    format!("parse failure: {}", err)
                }
            };
            assert!(stderr.write(msg.as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    println!("Job started at {}...\nLoading scene...", start_time);

    let camera = my_scene::mesh_sphere::get_camera(config.size.0, config.size.1, config.fov);
    let scene =  Arc::new(my_scene::mesh_sphere::get_scene());


    let scene_time = ::time::get_time().sec;
    println!("Scene loaded at {} ({}s)...", scene_time, scene_time - start_time);

    let renderer = raytracer::Renderer {
        shadow_samples: config.shadow_samples,
        pixel_samples: config.pixel_samples,
        // Number of tasks to spawn. Will use up max available cores.
        tasks: std::os::num_cpus()
    };

    // Still frame
    println!("Rendering with {} tasks...", renderer.tasks);
    let image_data = renderer.render(camera, scene);
    let render_time = ::time::get_time().sec;
    println!("Render done at {} ({}s)...\nWriting file...",
             render_time, render_time - scene_time);

    let out_file = format!("{}{}", config.output_file[], ".ppm");
    util::export::to_ppm(image_data, out_file[]);
    let export_time = ::time::get_time().sec;

    println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
             export_time, export_time - render_time,
             config.output_file[], export_time - start_time);
}
