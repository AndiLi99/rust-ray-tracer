mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ppm;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Lambertian, Scatterable};
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
            let scattered = record.material().scatter(ray, record);
            *scattered.attenuation() * ray_color(*scattered.ray(), world, depth - 1, rng)
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
    world.add(Box::new(Sphere::new(
        Point::point(0., 0., -1.),
        0.5,
        material::Material::Lambertian(Lambertian::new(Color::color(1.0, 0.0, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Point::point(0., -100.5, -1.),
        100.,
        material::Material::Lambertian(Lambertian::new(Color::color(0.0, 1.0, 0.0))),
    )));

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
