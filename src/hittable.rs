use crate::{material::Material, ray::Ray};
use glam::DVec3;
use std::ops::Range;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64,
}
impl HitRecord {
    pub fn with_face_normal(
        material: Material,
        p: DVec3,
        outward_normal: DVec3,
        t: f64,
        ray: &Ray,
        u: f64,
        v: f64,
    ) -> Self {
        let (front_face, normal) = HitRecord::calc_face_normal(ray, &outward_normal);
        HitRecord {
            material,
            p,
            normal,
            t,
            front_face,
            u,
            v,
        }
    }
    fn calc_face_normal(ray: &Ray, outward_normal: &DVec3) -> (bool, DVec3) {
        let front_face = ray.direction.dot(*outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }

    pub fn set_face_normal(
        material: Material,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        r: &Ray,
    ) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let u = 0.0;
        let v = 0.0;
        HitRecord {
            p: point,
            normal,
            t,
            front_face,
            material,
            u,
            v
        }
    }
}

// impl<T> Hittable for Vec<T>
// where
//     T: Hittable + Sync,
// {
//     fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
//         let (_closest, hit_record) = self.iter().fold((interval.end, None), |acc, item| {
//             if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
//                 (temp_rec.t, Some(temp_rec))
//             } else {
//                 acc
//             }
//         });

//         hit_record
//     }
// }
// use glam::DVec3;
// use std::ops::Range;

// use crate::ray::Ray;
// use crate::material::Material;

// #[derive(Clone)]
// pub struct HitRecord {
//     // pub p: DVec3,
//     // pub normal: DVec3,
//     // pub t: f64,
//     // pub material: Material,
//     // pub front_face: bool,
//     pub p: DVec3,
//     pub normal: DVec3,
//     pub t: f64,
//     pub front_face: bool,
//     pub material: Material,
//     pub u: f64,
//     pub v: f64,
// }

// impl HitRecord {
//     // fn with_face_normal(point: DVec3, outward_normal: DVec3, t: f64, ray: &Ray) -> Self {
//     //     Self { p: (), normal: (), t: (), front_face: () }
//     // }
    // fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
    //     self.front_face = r.direction.dot(outward_normal) < 0.0;
    //     if self.front_face {
    //         self.normal = outward_normal;
    //     } else {
    //         self.normal = -outward_normal;
    //     }
    // }

//     pub fn with_face_normal(
//         material: Material,
//         p: DVec3,
//         outward_normal: DVec3,
//         t: f64,
//         ray: &Ray,
//         u: f64,
//         v: f64
//     ) -> Self {
//         let (front_face, normal) =
//             HitRecord::calc_face_normal(
//                 ray,
//                 &outward_normal,
//             );
//         HitRecord {
//             material,
//             p,
//             normal,
//             t,
//             front_face,
//             u,
//             v
//         }
//     }
//     fn calc_face_normal(
//         ray: &Ray,
//         outward_normal: &DVec3,
//     ) -> (bool, DVec3) {
//         let front_face =
//             ray.direction.dot(*outward_normal) < 0.;
//         let normal = if front_face {
//             *outward_normal
//         } else {
//             -*outward_normal
//         };
//         (front_face, normal)
//     }
// }

// pub trait Hittable {
//     fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
// }

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn clear(&mut self) {
        self.objects = vec![]
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let (_closest, hit_record) = self.objects.iter().fold((interval.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
        // let oc: DVec3 = self.center - ray.origin;
        // let a: f64 = ray.direction.length_squared();
        // let half_b: f64 = ray.direction.dot(oc);
        // let c: f64 = oc.length_squared() - self.radius * self.radius;
        // let discriminant: f64 = half_b * half_b - a * c;
        // if discriminant < 0.0 {
        //     return None;
        // }

        // let sqrtd: f64 = discriminant.sqrt();

        // let mut root: f64 = (half_b - sqrtd) / a;
        // if !interval.contains(&root) {
        //     root = (half_b + sqrtd) / a;
        //     if !interval.contains(&root) {
        //         return None;
        //     }
        // }

        // rec.t = root.clone();
        // rec.p = ray.at(rec.t);
        // rec.normal = (rec.p - self.center) / self.radius;

        // return true;
    }
}

// impl<T> Hittable for Vec<T>
// where
//     T: Hittable + Sync,
// {
//     fn hit(
//         &self,
//         ray: &Ray,
//         interval: Range<f64>,
//     ) -> Option<HitRecord> {
//         let (_closest, hit_record) = self.iter().fold(
//             (interval.end, None),
//             |acc, item| {
//                 if let Some(temp_rec) =
//                     item.hit(ray, interval.start..acc.0)
//                 {
//                     (temp_rec.t, Some(temp_rec))
//                 } else {
//                     acc
//                 }
//             },
//         );

//         hit_record
//     }
// }