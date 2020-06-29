mod vector;
mod ray;

use ray::Ray;
use vector::Vec3;
fn main() {
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 384;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	println!("P3\n{} {}\n255", image_width, image_height);

	// define the viewport.
	let viewport_height =  2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;

	let origin = Vec3::new(0.0, 0.0, 0.0);
	let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
	let vertical = Vec3::new(0.0, viewport_height, 0.0);
	let lower_left_corner: Vec3 = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

	for j in (0..image_height).rev() {
		eprintln!("Scanlines remaining: {}", j);
		for i in 0..image_width {
			let u = i as f64 / (image_width - 1) as f64;
			let v = j as f64 / (image_width - 1) as f64;
			let direction = lower_left_corner + (horizontal * u) + (vertical * v) - origin;

			let ray = Ray::new(origin, direction);

			let pixel_colour = ray_colour(ray);
			write_colour(pixel_colour);
		}
	}
}

fn ray_colour(ray: Ray) -> Vec3 {
	let unit_direction = ray.direction.unit();

	let t = 0.5 * (unit_direction.y + 1.0);

	let r1: Vec3 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
	let r2: Vec3 = Vec3::new(0.5, 0.7, 1.0) * t;

	if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
		 Vec3::new(1.0, 1.0, 1.0)
	} else {
		r1 + r2
	}
}

fn write_colour(pixel_colour: Vec3) {
	let t_x = (255.999 * pixel_colour.x) as i64;
	let t_y = (255.999 * pixel_colour.y) as i64;
	let t_z = (255.999 * pixel_colour.z) as i64;
	println!("{} {} {}", t_x, t_y, t_z);
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
	let oc = r.origin - center;
	let a = r.direction.dot(r.direction);
	let b = 2.0 * oc.dot(r.direction);
	let c = oc.dot(oc) - radius * radius;

	let disciminant = b * b - (a * c * 4.0);

	disciminant > 0.0
}