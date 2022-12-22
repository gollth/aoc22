use anyhow::Result;
use clap::Parser;

use kiss3d::{
    camera::ArcBall,
    event::WindowEvent,
    light::Light,
    nalgebra::{Point3, Translation3},
    window::Window,
};
use std::{collections::HashMap, str::FromStr};

use eighteenth::{Coord, Lavablob};

/// Boiling Boulders: Solve the Aoc day 18 problem
#[derive(Debug, Parser)]
struct Options {
    /// Input file with the instructions
    #[clap(long, default_value = "sample.txt")]
    file: String,

    /// Start a 3D viewer to inspect the boulder
    #[clap(long)]
    visualize: bool,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let lava = Lavablob::from_str(&std::fs::read_to_string(args.file)?)?;

    let (mut min, mut max) = lava.bounds();
    min -= Coord::one();
    max += Coord::one();
    let cube = max - min;
    let cube_area = 2 * cube.x * cube.y + 2 * cube.x * cube.z + 2 * cube.y * cube.z;

    let water = Lavablob::from_iter(lava.region_around(&min, &min, &max).iter().cloned());

    println!(
        "Accessible surface area: {}",
        water.surface_area() as i32 - cube_area
    );

    if !args.visualize {
        return Ok(());
    }

    let center = cube / 2;
    let mut window = Window::new("Boiling Boulders");
    let mut camera = ArcBall::new(
        Point3::new(50., 0., 10.),
        Point3::new(center.x as f32, center.y as f32, center.z as f32),
    );
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.2, 0.2, 0.2);

    let mut cubes = HashMap::new();
    for coord in lava.iter() {
        let mut cube = window.add_cube(1., 1., 1.);
        cube.set_visible(false);
        cube.set_color(1., 0.2, 0.);
        cube.set_local_translation(Translation3::new(
            coord.x as f32,
            coord.y as f32,
            coord.z as f32,
        ));
        cubes.insert(coord, cube);
    }

    for coord in water.iter() {
        let mut cube = window.add_cube(1.0, 1.0, 1.0);
        cube.set_visible(false);
        cube.set_color(0., 0.8, 1.);
        cube.set_local_translation(Translation3::new(
            coord.x as f32,
            coord.y as f32,
            coord.z as f32,
        ));
        cubes.insert(coord, cube);
    }

    let mut slice = 0i32;
    while window.render_with_camera(&mut camera) {
        for event in window.events().iter() {
            if event.value == WindowEvent::Char('j') {
                if slice < max.x {
                    slice += 1;
                }
                for (coord, cube) in cubes.iter_mut() {
                    cube.set_visible(coord.x <= slice);
                }
            }
            if event.value == WindowEvent::Char('k') {
                if min.x < slice {
                    slice -= 1;
                }
                for (coord, cube) in cubes.iter_mut() {
                    cube.set_visible(coord.x <= slice);
                }
            }
        }
    }

    Ok(())
}
