mod camera;
mod hittable;
mod hittable_list;
mod ppm;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use ppm::write_color;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point, Vec3};

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

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

    // Camera

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point::point(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point::point(0., -100.5, -1.), 100.)));

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let u: f64 = x as f64 / (image_width as f64 - 1.);
            let v: f64 = y as f64 / (image_height as f64 - 1.);
            let ray: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color: Color = ray_color(ray, &world);

            write_color(color);
        }
    }
    eprintln!("\nDone.");
}
