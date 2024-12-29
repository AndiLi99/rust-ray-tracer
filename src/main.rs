mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ppm;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::sync::Arc;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal, Scatterable};
use crate::material::Material;
use ppm::write_color;
use ray::Ray;
use sphere::Sphere;
use utils::{lerp, random_double, random_double_between};
use vec3::{Color, Point, Vec3};

use rayon::prelude::*;
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
            match scattered {
                Some(s) => *s.attenuation() * ray_color(*s.ray(), world, depth - 1, rng),
                None => Color::black(),
            }
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
    let image_width: i64 = 1200;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel: i64 = 500;
    let max_depth: i32 = 50;

    // Camera

    let v_fov: f64 = 20.;

    let lookfrom = Point::point(13., 2., 3.);
    let lookat = Point::point(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);

    let defocus_angle = 0.6;
    let focus_distance = 10.;

    let camera: Camera = Camera::new(aspect_ratio, v_fov, defocus_angle, focus_distance, lookfrom, lookat, vup);

    // Materials
    let ground = material::Material::Lambertian(Lambertian::new(Color::color(0.5, 0.5, 0.5)));
    let mat_1 = material::Material::Dielectric(Dielectric::new(1.5));
    let mat_2 = material::Material::Lambertian(Lambertian::new(Color::color(0.1, 0.2, 0.5)));
    let mat_3 = material::Material::Metal(Metal::new(Color::color(0.7, 0.6, 0.5), 0.));

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point::point(0., -1000., 0.),
        1000.,
        ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::point(0., 1., 0.),
        1.,
        mat_1,
    )));
    world.add(Arc::new(Sphere::new(
        Point::point(-4., 1., 0.),
        1.,
        mat_2,
    )));
    world.add(Arc::new(Sphere::new(
        Point::point(4., 1., 0.),
        1.,
        mat_3,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double();
            let center = Point::point(0.9*utils::random_double() + (a as f64), 0.2, 0.9*random_double() + (b as f64));

            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = random_double_between(0., 0.5);
                    let mat = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(center, 0.2, mat_1)));
                }
            }
        }
    }

    // Render in parallel

    let mut buffer = vec![String::new(); (image_width * image_height) as usize];

    buffer.par_iter_mut().enumerate().for_each(|(index, pixel_data)| {
        let mut rng = rand::thread_rng();
        let x = (index as i64) % image_width;
        let y = image_height - 1 - (index as i64) / image_width; // Reverse y-axis

        let mut pixel_color = Color::black();

        for _ in 0..samples_per_pixel {
            let u: f64 = (x as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.);
            let v: f64 = (y as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.);
            let ray: Ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray_color(ray, &world, max_depth, &mut rng);
        }

        // Write the final color to the buffer
        *pixel_data = write_color(pixel_color, samples_per_pixel);
    });
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    println!("{}", buffer.join("\n"));
    eprintln!("\nDone.");
}
