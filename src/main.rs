mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

extern crate rand;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::Hittable;
use camera::Camera;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    let rng = rand::thread_rng();
    raytrace(rng)
}

fn colour<T: Hittable>(r: Ray, world: &T, rng: ThreadRng) -> Vec3 {
    match world.hit(&r, 0.001, std::f64::MAX) {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_in_unit_sphere(rng);
            let diffuse_ray = Ray { origin: hit_record.p, direction: target - hit_record.p };
            0.5 * colour(diffuse_ray, world, rng)
        }
        None => {
            let unit_direction = Vec3::unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn random_double(mut rng: ThreadRng) -> f64 {
    rng.gen_range(0.0, 1.0)
}

fn random_in_unit_sphere(rng: ThreadRng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3(random_double(rng), random_double(rng), random_double(rng)) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn raytrace(rng: ThreadRng) {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let camera = Camera {
        lower_left_corner: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
        origin: Vec3(0.0, 0.0, 0.0)
    };

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
        eprintln!("y = {}", j);
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f64) + random_double(rng)) / (nx as f64);
                let v = ((j as f64) + random_double(rng)) / (ny as f64);
                let ray = camera.get_ray(u, v);
                col += colour(ray, &world, rng);
            }
            col /= ns as f64;
            col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
