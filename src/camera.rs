use crate::ray::Ray;
use crate::hittable::Hittable;

use glam::DVec3;
use itertools::Itertools;
use rand::*;
use std::{fs, io, ops::Range};
pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,

    image_height: u32,

    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_center: DVec3,

    viewport_u: DVec3,
    viewport_v: DVec3,

    max_value: u8,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,

    viewport_upper: DVec3,
    pixel00_loc: DVec3,

    samples_per_pixel: u32,

    max_depth: u32,

    vfov: f64,
    theta: f64,
    h: f64,

    lookfrom: DVec3,
    lookat: DVec3,
    vup: DVec3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
}

impl Camera {
    pub fn new(lookfrom: DVec3, lookat: DVec3, vup: DVec3) -> Self {
        let aspect_ratio: f32 = 4.0 / 3.0;
        let image_width: u32 = 400;

        // Calcul de la hauteur de l'image en fonction du ratio et de sa largeur
        let image_height: u32 = image_width / aspect_ratio as u32;

        // Placement et orientation de la caméra
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Camera
        let vfov = 90.0; // zoom
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let focal_length: f64 = (lookfrom - lookat).length();
        let viewport_height: f64 = 2.0 * h * focal_length;
        let viewport_width: f64 = viewport_height * (image_width / image_height) as f64;
        let camera_center: DVec3 = lookfrom;

        // Calcul des vecteurs sur les bords horizontaux et sous les bords verticaux de la vue de caméra
        let viewport_u: DVec3 = u * viewport_width;
        let viewport_v: DVec3 = -v * viewport_height;

        let max_value: u8 = 255;
        // Calcul des vecteurs delta verticaux et horizontaux pixel par pixel
        let pixel_delta_u: DVec3 = viewport_u / image_width as f64;
        let pixel_delta_v: DVec3 = viewport_v / image_height as f64;

        // Calcul de la position du pixel en haut à gauche (premier pixel de la vue caméra)
        let viewport_upper_left: DVec3 =
            camera_center - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Niveau d'anti-aliasing
        let samples_per_pixel = 500;

        let max_depth = 50;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            max_value,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper: viewport_upper_left,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
            vfov,
            theta,
            h,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
        }
    }

    pub fn render<T>(&self, world: T) -> io::Result<()>
    where
        T: Hittable,
    {
        // Rendering
        let pixels = indicatif::ProgressIterator::progress_count(
            (0..self.image_height).cartesian_product(0..self.image_width),
            self.image_height as u64 * self.image_width as u64,
        )
        .map(|(y, x)| {
            let scale_factor = (self.samples_per_pixel as f64).recip();

            let multisampled_pixel_color = (0..self.samples_per_pixel)
                .into_iter()
                .map(|_| {
                    self.get_ray(x, y).color(self.max_depth as i32, &world) * 255.0 * scale_factor
                })
                .sum::<DVec3>();

            format!(
                "{} {} {}",
                multisampled_pixel_color.x as u32,
                multisampled_pixel_color.y as u32,
                multisampled_pixel_color.z as u32
            )
        })
        .join("\n");
        fs::write(
            "output_test_cubex500.ppm",
            format!(
                "P3
        {} {}
    {}
    {}
    ",
                self.image_width, self.image_height, self.max_value, pixels
            ),
        )?;
        Ok(())
    }

    pub fn render_to_disk<T>(&self, world: T) -> io::Result<()>
    where
        T: Hittable + std::marker::Sync,
    {
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .map(|(y, x)| {
                let scale_factor = (self.samples_per_pixel as f64).recip();

                let multisampled_pixel_color = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        self.get_ray(x, y).color(self.max_depth as i32, &world)
                            * 255.0
                            * scale_factor
                    })
                    .sum::<DVec3>()
                    * scale_factor;

                // * 256.
                let color = DVec3 {
                    x: multisampled_pixel_color.x.sqrt(),
                    y: multisampled_pixel_color.y.sqrt(),
                    z: multisampled_pixel_color.z.sqrt(),
                }
                .clamp(DVec3::splat(0.), DVec3::splat(0.999))
                    * 256.;
                format!("{} {} {}", color.x as u8, color.y as u8, color.z as u8)
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(
            "nico_test.ppm",
            format!(
                "P3
{} {}
{}
{pixels}
",
                self.image_width, self.image_height, self.max_value
            ),
        )
    }

    fn sample_square(&self) -> DVec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    // Va récupérer une position aléatoire autour du pixel pointé (sample_square) et renvoyer un rayon
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + offset;
        let ray_direction = pixel_sample - self.camera_center;

        return Ray::new(self.camera_center, ray_direction);
    }

    // Si besoin cf section 7
    // fn color<T>(r: &Ray, world: &T) -> DVec3 {

    // }
}
