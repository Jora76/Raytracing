use crate::{material, quad};
use glam::DVec3;
use rand::Rng;

pub struct Cube {
    pub faces: Vec<quad::Quad>,
}

impl Cube {
    pub fn new(center: DVec3, size: f64, material: material::Material) -> Self {
        let half_size = size / 2.0;

        // Définir les sommets du cube
        let p0 = center + DVec3::new(-half_size, -half_size, -half_size);
        let p1 = center + DVec3::new(half_size, -half_size, -half_size);
        let p2 = center + DVec3::new(half_size, half_size, -half_size);
        let p3 = center + DVec3::new(-half_size, half_size, -half_size);
        let p4 = center + DVec3::new(-half_size, -half_size, half_size);
        let p5 = center + DVec3::new(half_size, -half_size, half_size);
        let p6 = center + DVec3::new(half_size, half_size, half_size);
        let p7 = center + DVec3::new(-half_size, half_size, half_size);

        let material_center = material::Material::Lambertian {
            albedo: DVec3::new(0.8, 0.2, 0.5),
        };

        let material_sup = material::Material::Lambertian {
            albedo: DVec3::new(0.8, 0.8, 0.5),
        };

        // Définir les faces du cube
        let faces = vec![
            // Face avant
            quad::Quad::new(p0, p1 - p0, p3 - p0, material.clone()),
            // Face arrière
            quad::Quad::new(p4, p5 - p4, p7 - p4, material_center.clone()),
            // Face gauche
            quad::Quad::new(p0, p4 - p0, p3 - p0, material.clone()),
            // Face droite
            quad::Quad::new(p1, p5 - p1, p2 - p1, material_center.clone()),
            // Face supérieure
            quad::Quad::new(p3, p7 - p3, p2 - p3, material_sup.clone()),
            // Face inférieure
            quad::Quad::new(p0, p4 - p0, p1 - p0, material_center.clone()),
        ];

        Self { faces }
    }
}
