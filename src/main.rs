mod camera;
mod hittable;
mod hittable_list;
mod ppm;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use ppm::write_color;
use ray::Ray;
use sphere::Sphere;
use utils::lerp;
use vec3::{Color, Point, Vec3};

use rand::rngs::ThreadRng;
use rand::Rng;

fn ray_color<T: Hittable>(ray: Ray, world: &T, depth: i32, rng: &mut ThreadRng) -> Color {
    // Due to subtle bug with floating point rounding, the calculated intersection point 
    // which is the origin of the next ray can be sligthly off.
    // That point could be slightly above or below the surface of the sphere, 
    // if it is below, it could hit the same sphere again.
    // Solution: make the ray ignore hits within a small distance
    let hit_record = world.hit(ray, 0.001, f64::INFINITY);

    if depth <= 0 {
        return Color::black();
    }

    match hit_record {
        Some(record) => {
            let direction = record.normal() + random_vec_on_unit_sphere(rng);
            0.5 * ray_color(
                Ray::new(record.p(), direction),
                world,
                depth - 1,
                rng,
            )
        }
        None => {
            let unit_direction: Vec3 = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            let white: Color = Vec3(1.0, 1.0, 1.0);
            let blue: Color = Vec3(0.5, 0.7, 1.0);
            lerp(t, white, blue)
        }
    }
}

fn map_to_range(v: Vec3, min: f64, max: f64) -> Vec3 {
    min + (max - min) * v
}
fn random_vec_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    for _ in 0..10000 {
        let p: Vec3 = map_to_range(rng.gen(), -1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
    panic!("Could not find a random vector in the unit sphere!")
}
fn random_vec_on_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    random_vec_in_unit_sphere(rng).unit_vector()
}
fn random_vec_on_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3{
    let on_unit_sphere = random_vec_on_unit_sphere(rng);
    if on_unit_sphere.dot(normal) > 0. {
        on_unit_sphere
    } else {    
        -on_unit_sphere
    }
}
fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0; // width divided by height
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel: i64 = 100;
    let max_depth: i32 = 50;

    // Camera

    let viewport_height: f64 = 2.0;
    let focal_length: f64 = 1.0;

    let camera: Camera = Camera::new(aspect_ratio, viewport_height, focal_length);
    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point::point(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point::point(0., -100.5, -1.), 100.)));

    // rng
    let mut rng = rand::thread_rng();

    // Render

    let mut vec: Vec<String> = Vec::new();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}    ", y);
        for x in 0..image_width {
            let mut pixel_color: Color = Color::black();
            for _ in 0..samples_per_pixel {
                let u: f64 = (x as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.);
                let v: f64 = (y as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.);
                let ray: Ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, max_depth, &mut rng);
            }

            vec.push(write_color(pixel_color, samples_per_pixel));
        }
    }
    println!("{}", vec.join("\n"));
    eprintln!("\nDone.");
}
