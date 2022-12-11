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

use rand::Rng;

fn ray_color<T: Hittable>(ray: Ray, world: &T) -> Color {
    let hit_record = world.hit(ray, 0., f64::INFINITY);

    match hit_record {
        Some(record) => 0.5 * (record.normal() + Color::color(1., 1., 1.)),
        None => {
            let unit_direction: Vec3 = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            let white: Color = Vec3(1.0, 1.0, 1.0);
            let blue: Color = Vec3(0.5, 0.7, 1.0);
            lerp(t, white, blue)
        }
    }
}
fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0; // width divided by height
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel: i64 = 100;

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
                pixel_color = pixel_color + ray_color(ray, &world);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
