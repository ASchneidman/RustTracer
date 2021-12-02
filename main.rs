extern crate image;
extern crate nalgebra as na;
extern crate rand;

use image::{ImageBuffer, RgbImage};
use na::{Vector3};
use std::cmp;
use rand::{Rng, thread_rng};


struct Sphere<'a> {
    center: &'a Vector3<f32>,
    radius: &'a f32
}

struct Ray<'a> {
    origin: &'a Vector3<f32>,
    direction: &'a Vector3<f32>
}

struct Camera<'a> {
    focal_length: &'a f32,
    sensor_width: &'a u32,
    sensor_height: &'a u32,
    sensor_buffer: &'a mut RgbImage,
    px_size: &'a f32 
}

fn ray_sphere_intersection<'a>(ray: &'a Ray, sphere: &'a Sphere) -> f32 {
    let L = sphere.center - ray.origin;

    let tca = L.dot(ray.direction);
    if (tca < 0.0) {
        return -1.0;
    } 

    let d = L.dot(&L) - tca * tca;
    let r2 = sphere.radius * sphere.radius;
    if (d > r2) {
        return -1.0;
    }

    let thc = (r2 - d).sqrt();

    let t0 = tca - thc;
    let t1 = tca + thc;

    if (t0 < 0.0) {
        if (t1 < 0.0) {
            return -1.0;
        }
        return t1;
    } else {
        if (t1 < 0.0) {
            return t0;
        }
        return t0.min(t1);
    }

}

fn raytrace<'a>(sphere: &'a Sphere, camera: &'a mut Camera) {
    let upper_left_x = -camera.px_size * (*camera.sensor_width as f32) / 2.0;
    let upper_left_y = camera.px_size * (*camera.sensor_height as f32) / 2.0;

    let supersample = 8;

    // Assume camera is at (0, 0, 0) looking down negative z axis
    for x in 0..*camera.sensor_width {
        for y in 0..*camera.sensor_height {
            let px_x = upper_left_x + (x as f32) * camera.px_size;
            let px_y = upper_left_y - (y as f32) * camera.px_size;

            let mut ave_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..supersample {
                let x_noise = thread_rng().gen_range(-0.5..0.5) * *camera.px_size;
                let y_noise = thread_rng().gen_range(-0.5..0.5) * *camera.px_size;


                let sample_x = px_x + x_noise;
                let sample_y = px_y + y_noise;
                // Cast ray from this pixel
                let pixel = Vector3::new(
                    sample_x,
                    sample_y,
                    *camera.focal_length
                );

                // Since all rays pass through origin and sensor is in front
                // of camera, this ray has direction (pixel - (0, 0, 0))
                let ray = Ray {
                    origin: &Vector3::new(0.0, 0.0, 0.0),
                    direction: &pixel.normalize()
                };

                let hit_t = ray_sphere_intersection(&ray, sphere);

                if (hit_t >= 0.0) {
                    let hit_pt = ray.origin + ray.direction * hit_t;
                    let normal = (hit_pt - sphere.center).normalize();

                    ave_color = ave_color + normal.add_scalar(1.0) * 0.5;
                }
            }
            ave_color = ave_color / (supersample as f32);

            let pixel = camera.sensor_buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb([(ave_color.x * 255.0) as u8, (ave_color.y * 255.0) as u8, (ave_color.z * 255.0) as u8])

        }
    }
}

fn main() {
    let dim_x = 1024;
    let dim_y = 1024;

    let camera = &mut Camera {
        focal_length: &1.0,
        sensor_width: &dim_x,
        sensor_height: &dim_y,
        sensor_buffer: &mut ImageBuffer::new(dim_x, dim_y),
        px_size: &0.001
    };

    let sphere = & Sphere {
        center: &Vector3::new(0.0, 0.0, 10.0),
        radius: &1.0
    };

    raytrace(sphere, camera);
    camera.sensor_buffer.save("output.png").unwrap();
}