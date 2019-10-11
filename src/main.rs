pub mod vec3;
pub mod ray;

use vec3::Vec3;
use ray::Ray;

fn main() {
    raytrace()
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(r.direction, r.direction);
    let b = 2.0 * Vec3::dot(oc, r.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn colour(r: Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &r) {
        Vec3(1.0, 0.0, 0.0)
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}

fn raytrace() {
    let nx = 200;
    let ny = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray {
                origin,
                direction
            };
            let col = colour(r);
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
