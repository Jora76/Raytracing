use crate::ray::*;
use crate::hittable::HitRecord;

use glam::DVec3;

#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3 },
}
pub struct Scattered {
    pub attenuation: DVec3,
    pub scattered: Ray,
}
impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hit_record: HitRecord,
    ) -> Option<Scattered> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record
                    .normal
                    + random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.abs_diff_eq(
                    DVec3::new(0., 0., 0.),
                    1e-8,
                ) {
                    scatter_direction = hit_record.normal;
                }

                let scattered = Ray {
                    origin: hit_record.p,
                    direction: scatter_direction,
                };

                Some(Scattered {
                    attenuation: *albedo,
                    scattered,
                })
            }
            Material::Metal { albedo } => {
                let reflected: DVec3 = reflect(
                    r_in.direction.normalize(),
                    hit_record.normal,
                );
                Some(Scattered {
                    attenuation: *albedo,
                    scattered: Ray {
                        origin: hit_record.p,
                        direction: reflected,
                    },
                })
            }
            _ => None,
        }
    }
}

fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2. * v.dot(n) * n
}