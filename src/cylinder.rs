// use crate::{
//     hittable::{HitRecord, Hittable},
//     material::Material,
//     ray::Ray,
// };
// use glam::{DVec3, Vec3Swizzles};
// use std::ops::Range;

// pub struct Cylinder {
//     pub start: DVec3,
//     pub end: DVec3,
//     pub radius: f64,
//     pub material: Material,
// }
// // return Some(HitRecord::with_face_normal(
// //     self.material.clone(),
// //     ray.at(tF),
// //     normal,
// //     tF,
// //     ray,
// // ));
// impl Hittable for Cylinder {
//     fn hit(
//         &self,
//         ray: &Ray,
//         interval: Range<f64>,
//     ) -> Option<HitRecord> {
//         let pa = self.start;
//         let pb = self.end;
//         let ca: DVec3 = pb - pa;
//         let oc: DVec3 = ray.origin - pa;

//         let caca: f64 = ca.dot(ca);
//         let card: f64 = ca.dot(ray.direction);
//         let caoc: f64 = ca.dot(oc);

//         let a: f64 = caca - card * card;
//         let b: f64 =
//             caca * oc.dot(ray.direction) - caoc * card;
//         let c: f64 = caca * oc.dot(oc)
//             - caoc * caoc
//             - self.radius * self.radius * caca;
//         let mut h: f64 = b * b - a * c;

//         if h < 0. {
//             return None;
//         }

//         h = h.sqrt();
//         let mut d: f64 = (-b - h) / a;

//         let y: f64 = caoc + d * card;
//         if y > 0. && y < caca && interval.contains(&d) {
//             let normal = (oc + d * ray.direction
//                 - ca * y / caca)
//                 / self.radius;
//             return Some(HitRecord::set_face_normal(
//                 self.material.clone(),
//                 ray.at(d),
//                 // oc + d * ray.direction,
//                 normal,
//                 d,
//                 ray,
//             ));
//         }

//         d = ((if y < 0. { 0. } else { caca }) - caoc)
//             / card;

//         if (b + a * d).abs() < h && interval.contains(&d) {
//             let normal =
//                 (ca * y.signum() / caca).normalize();
//             return Some(HitRecord::set_face_normal(
//                 self.material.clone(),
//                 ray.at(d),
//                 normal,
//                 d,
//                 ray,
//             ));
//         } else {
//             return None;
//         }
//     }
// }

// // float iCylinder( in vec3 ro, in vec3 rd, in vec2 distBound, inout vec3 normal,
// //     in vec3 pa, in vec3 pb, float ra ) {

// // }

use crate::{ray::Ray, material::Material, hittable::{Hittable, HitRecord}};
use glam::DVec3;
use std::ops::Range;

pub struct Cylinder {
    pub base_center: DVec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    fn get_uv(p: &DVec3) -> (f64, f64) {
        // Calcul basique des coordonnées UV pour un cylindre
        let theta = f64::atan2(p.z, p.x);
        let u = 1.0 - (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (p.y + 1.0) / 2.0; // Adaptez en fonction de la hauteur
        (u, v)
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.base_center;

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * (oc.x * ray.direction.x + oc.z * ray.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let mut t2 = (-b + sqrt_discriminant) / (2.0 * a);

        // Vérification que les intersections sont dans les limites du cylindre
        let mut hit_point1 = ray.at(t1);
        let mut hit_point2 = ray.at(t2);

        if (hit_point1.y >= self.base_center.y && hit_point1.y <= self.base_center.y + self.height) ||
            (hit_point2.y >= self.base_center.y && hit_point2.y <= self.base_center.y + self.height) {
            
            // Prioriser t1
            let t = if interval.contains(&t1) { t1 } else if interval.contains(&t2) { t2 } else { return None; };
            let hit_point = ray.at(t);

            // Calcul de la normale à la surface du cylindre
            let outward_normal = DVec3::new(hit_point.x - self.base_center.x, 0.0, hit_point.z - self.base_center.z).normalize();

            // Calcul des coordonnées UV
            let (u, v) = Cylinder::get_uv(&hit_point);

            // Création du HitRecord
            return Some(HitRecord::with_face_normal(
                self.material.clone(),
                hit_point,
                outward_normal,
                t,
                ray,
                u,
                v,
            ));
        }

        None
    }
}