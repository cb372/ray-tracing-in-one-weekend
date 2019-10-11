mod vec3;
mod ray;
mod hittable;
mod sphere;


use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::Hittable;

fn main() {
    raytrace()
}

fn colour<T: Hittable>(r: Ray, world: &T) -> Vec3 {
    match world.hit(&r, 0.0, std::f64::MAX) {
        Some(hit_record) => {
            0.5 * Vec3(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0
            )
        }
        None => {
            let unit_direction = Vec3::unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
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

    let sphere = Sphere {
        centre: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let background = Sphere {
        centre: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    };
    let world = vec![sphere, background];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray {
                origin,
                direction
            };
            let col = colour(r, &world);
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
