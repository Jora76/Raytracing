use glam::DVec3;
use rand::*;

use crate::hittable::Hittable;
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn color<T>(&self, depth: i32, world: &T) -> DVec3
    where
        T: Hittable,
    {
        if depth <= 0 {
            return DVec3::new(0., 0., 0.);
        }

        if let Some(rec) = world.hit(&self, (0.001)..f64::INFINITY) {
            let scattered = rec.material.scatter(self, rec.clone());
            if let Some(scattered) = scattered {
                // println!("scattered: {:?}", scattered.attenuation);
                return apply_gamma(scattered.attenuation * scattered.scattered.color(depth - 1, world), 1.);

            }
            return DVec3::new(0., 0., 0.);
            // let direction = rec.normal + random_unit_vector();
            // // return 0.5 * (rec.normal + DVec3::new(1.0, 1.0, 1.0));
            // let ray = Ray {
            //     origin: rec.p,
            //     direction,
            // };
            // let reflectance = 0.5; // gère la réflection de la lumière plus c'est bas moins il y a de réflection
            // return reflectance * ray.color(depth - 1, world);
        }
        let unit_direction: DVec3 = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }

    // fn color(&self) -> DVec3 {
    //     let t: f64 = hit_sphere(&DVec3::new(0.0, 0.0, -1.0), 0.5, self);
    //     // return DVec3::new(1.0, 0.0, 0.0);
    //     if t > 0.0 {
    //         let N: DVec3 = (self.at(t) - DVec3::new(0.0, 0.0, -1.0)).normalize();
    //         return 0.5 * (N + 1.0);
    //     }
    //     let unit_direction: DVec3 = self.direction.normalize();
    //     let a = 0.5 * (unit_direction.y + 1.0);
    //     return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    // }
}

fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = DVec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> DVec3 {

    return random_in_unit_sphere().normalize(); 
}


fn apply_gamma(color: DVec3, gamma: f64) -> DVec3 {
    DVec3::new(
        color.x.powf(1.0 / gamma),
        color.y.powf(1.0 / gamma),
        color.z.powf(1.0 / gamma),
    )
}
// fn random_in_hemisphere(normal: DVec3) -> DVec3 {
//     let in_unit_sphere = random_unit_vector();
//     if in_unit_sphere.dot(normal) > 0.0 {
//         return in_unit_sphere;
//     } else {
//         return -in_unit_sphere;
//     }
// }
