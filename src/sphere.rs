use crate::hittable::*;
use crate::material::Material;
use crate::ray::*;

use std::ops::Range;
use glam::DVec3;

use std::f64::consts::PI;
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc: DVec3 = self.center - ray.origin;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = discriminant.sqrt();
        // println!("sqrtd: {}", sqrtd);

        let mut root: f64 = (half_b - sqrtd) / a;
        if !interval.contains(&root) {
            root = (half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }
        let t = root.clone();
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        // let material = &self.material;
        let (u, v) = self.get_sphere_uv(normal);
        // let rec = HitRecord { p, t, material: self.material.clone(), normal, front_face: false };
        // let rec = HitRecord::with_face_normal(self.material, p, normal, t, ray, u, v)

        let rec = HitRecord::with_face_normal(
            self.material.clone(),
            p,
            normal,
            t,
            ray,
            u,
            v,
        );
        
        return Some(rec);
    }
}

impl Sphere {
    fn get_sphere_uv(&self, p: DVec3) -> (f64, f64) {

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;
        (u, v)
    }
}

// fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> f64 {
//     let oc: DVec3 = *center - ray.origin;
//     let a: f64 = ray.direction.length_squared();
//     let half_b: f64 = ray.direction.dot(oc);
//     let c: f64 = oc.length_squared() - radius * radius;
//     let discriminant: f64 = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         return -1.0;
//     }
//     return (half_b - discriminant.sqrt()) / a;
// }