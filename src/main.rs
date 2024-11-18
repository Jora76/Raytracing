use glam::DVec3;
use rt::camera::Camera;
use rt::material;
use std::io;

use rt::cube::Cube;
use rt::cylinder::Cylinder;
use rt::hittable::*;
use rt::quad::Quad;
use rt::sphere::*;

//---------------------------------------------------SPHERES--------------------------------------------------------------------------------
// fn main() -> io::Result<()> {
//     let mut world = HittableList { objects: vec![] };
//     // let mut world = vec![];

//     let material_ground = material::Material::Lambertian {
//         albedo: DVec3::new(0.8, 0.8, 0.0),
//     };

//     let material_center = material::Material::Lambertian {
//         albedo: DVec3::new(0.1, 0.2, 0.5),
//     };

//     let material_left = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.8, 0.8),
//     };

//     let material_right = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.6, 0.2),
//     };

//     world.add(Sphere {
//         center: DVec3::new(0.0, -100.5, -1.0),
//         radius: 100.0,
//         material: material_ground,
//     });

//     world.add(Sphere {
//         center: DVec3::new(0.0, 0.0, -1.2),
//         radius: 0.5,
//         material: material_center,
//     });

//     world.add(Sphere {
//         center: DVec3::new(-1.0, 0.0, -1.0),
//         radius: 0.5,
//         material: material_left,
//     });

//     world.add(Sphere {
//         center: DVec3::new(1.0, 0.0, -1.0),
//         radius: 0.5,
//         material: material_right,
//     });

//     let camera = Camera::new(
//         DVec3::new(-2., 2., 1.),
//         DVec3::new(0., 0., -1.),
//         DVec3::new(0., 1., 0.),
//     );
//     camera.render(world)?;

//     Ok(())
// }

// ---------------------------------------------------SPHERE & CUBE------------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let mut world = HittableList { objects: vec![] };

    let material_ground = material::Material::Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    };

    world.add(Sphere {
        center: DVec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    });

    let center = DVec3::new(0.0, 0.0, 0.0);
    let size = 1.0;

    let material_left = material::Material::Metal {
        albedo: DVec3::new(0.8, 0.8, 0.8),
    };

    world.add(Sphere {
        center: DVec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    });

    let material_center = material::Material::Lambertian {
        albedo: DVec3::new(0.1, 0.2, 0.5),
    };

    let cube = Cube::new(center, size, material_center);
    // Ajoutez les faces du cube au monde
    for face in cube.faces {
        world.add(face);
    }

    let camera = Camera::new(
        DVec3::new(-2.5, 1., 1.),
        DVec3::new(0., -0.5, -1.),
        DVec3::new(0., 1., 0.),
    );

    camera.render(world)?;

    Ok(())
}

// --------------------------------------------------------------------------------------------------------------------------------------------

// use std::io;

// fn main() -> io::Result<()> {
//     let mut world = HittableList { objects: vec![] };
//     // let mut world = vec![];

//     let material_ground = material::Material::Lambertian {
//         albedo: DVec3::new(0.8, 0.8, 0.0).into(),
//     };
//     let material_center = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.8, 0.8).into(),
//     };

//     world.add(Sphere {
//         center: DVec3::new(0.0, -100.5, -1.0),
//         radius: 100.0,
//         material: material_ground,
//     });
//     world.add(Cylinder {
//         base_center: DVec3 {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         },
//         radius: 0.5,
//         height: 2.0,
//         material: material_center,
//     });

//     let material_left = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.8, 0.8),
//     };
//     world.add(Sphere {
//         center: DVec3::new(-1.0, 0.0, 0.0),
//         radius: 0.6,
//         material: material_left,
//     });

//     let material_right = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.8, 0.8),
//     };
//     world.add(Sphere {
//         center: DVec3::new(1.0, 0.0, 0.0),
//         radius: 0.6,
//         material: material_right,
//     });

    
//     let material_up = material::Material::Metal {
//         albedo: DVec3::new(0.8, 0.8, 0.8),
//     };
//     world.add(Sphere {
//         center: DVec3::new(0.0, 2.0, 0.0),
//         radius: 0.5,
//         material: material_up,
//     });

//     // let camera = Camera::init()
//     //     .image_width(600)
//     //     .aspect_ratio(16.0 / 9.0)
//     //     .look_from(DVec3::new(1., 1., 10.))
//     //     .look_at(DVec3::NEG_Z)
//     //     .vup(DVec3::Y)
//     //     // .focus_dist(10.0)
//     //     // .defocus_angle(0.0)
//     //     .samples_per_pixel(100)
//     //     .max_depth(50)
//     //     .build();

//     // camera.render_to_disk("cylinder", world)?;
//     let camera = Camera::new(DVec3::new(0., 4., 4.), DVec3::NEG_Z, DVec3::Y);
//     camera.render(world)?;

//     Ok(())
// }
