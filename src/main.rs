pub mod vec3;

use vec3::Vec3;

fn main() {
    hello_world()
}

fn hello_world() {
    let nx = 200;
    let ny = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3(
                (i as f64) / (nx as f64),
                (j as f64) / (ny as f64),
                0.2
            );
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
