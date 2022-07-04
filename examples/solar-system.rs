use oberth::{System, Id, SimpleBody, Mass, Pos, Vel, Radius, Vec3};
use std::time::Duration;
use nalgebra::{Vector3, UnitQuaternion};
use kiss3d::{event::{Key, Action}, window::Window, light::Light};
use rand::{thread_rng, Rng};

const SCALE: f64 = 0.000_000_000_1;

fn main() {
    let mut sys = System::new();

    let mut body = |parent: Option<Id>, dist, vel, mass, radius| {
        let (rpos, rvel) = parent
            .map(|parent| (sys.get::<Pos>(parent).unwrap().0, sys.get::<Vel>(parent).unwrap().0))
            .unwrap_or_default();
        sys.add(SimpleBody {
            pos: Pos(rpos + Vec3::new(dist, 0.0, 0.0)),
            vel: Vel(rvel + Vec3::new(0.0, 0.0, vel)),
            mass: Mass(mass),
            radius: Radius(radius),
        })
    };

    let au = 152_100_000_000.0;

    let sun = body(
        None,
        au * 0.0, // dist
        0.0, // vel
        1_988_500_000_000_000_000_000_000_000_000.0, // mass
        695_700_000.0, // radius
    );
    let mercury = body(
        Some(sun),
        au * 0.466697, // dist
        47_360.0, // vel
        330_110_000_000_000_000_000_000.0, // mass
        4_880_000.0, // radius
    );
    let venus = body(
        Some(sun),
        au * 0.728213, // dist
        35_020.0, // vel
        4_867_500_000_000_000_000_000_000.0, // mass
        6_051_800.0, // radius
    );
    let earth = body(
        Some(sun),
        au * 1.0, // dist
        29_780.0, // vel
        5_972_370_000_000_000_000_000_000.0, // mass
        6_371_000.0, // radius
    );
    let moon = body(
        Some(earth),
        405_400_000.0, // dist
        1_022.0, // vel
        73_420_000_000_000_000_000_000.0, // mass
        1_737_400.0, // radius
    );
    let mars = body(
        Some(sun),
        au * 1.66621, // dist
        24_070.0, // vel
        641_710_000_000_000_000_000_000.0, // mass
        3_376_200.0, // radius
    );
    let jupiter = body(
        Some(sun),
        au * 5.4570, // dist
        13_070.0, // vel
        1_898_200_000_000_000_000_000_000_000.0, // mass
        71_492_000.0, // radius
    );

    let mut bodies = vec![
        (sun, [1.0, 0.8, 0.0]),
        (mercury, [0.8, 0.8, 0.0]),
        (venus, [1.0, 0.9, 0.5]),
        (earth, [0.0, 0.7, 1.0]),
        (mars, [1.0, 0.5, 0.3]),
        (moon, [0.5, 0.5, 0.6]),
        (jupiter, [1.0, 0.3, 0.1]),
    ];

    // for _ in 0..50 {
    //     let scale = thread_rng().gen::<f64>().powf(3.0) * 0.1;
    //     let id = sys.add(SimpleBody {
    //         pos: Pos(-earth_pos * 1.5 + Vec3::new(
    //             thread_rng().gen_range(-1_000_000_000.0..1_000_000_000.0),
    //             thread_rng().gen_range(-1_000_000_000.0..1_000_000_000.0),
    //             thread_rng().gen_range(-1_000_000_000.0..1_000_000_000.0),
    //         )),
    //         vel: Vel(earth_vel * -thread_rng().gen_range(0.35..1.15)),
    //         mass: Mass(73_420_000_000_000_000_000_000.0 * scale),
    //         radius: Radius(1_737_400.0 * scale),
    //     });
    //     bodies.push((id, [200.0; 3]));
    // }

    let mut window = Window::new("Solar System");
    window.set_light(Light::StickToCamera);

    let mut body_shapes = bodies
        .into_iter()
        .map(|(id, [r, g, b])| {
            let mut body = window.add_sphere((sys.get::<Radius>(id).unwrap().0 * SCALE).powf(0.35) as f32 * 15.0);
            body.set_color(r, g, b);
            (body, id)
        })
        .collect::<Vec<_>>();

    while window.render() {
        let frame = sys.get::<Pos>(sun).unwrap().0;

        for (body_shape, id) in &mut body_shapes {
            body_shape.set_local_translation(((sys.get::<Pos>(*id).unwrap().0 - frame) * SCALE).map(|e| e as f32).into_array().into());
        }

        if window.get_key(Key::Space) != Action::Press {
            let speed = if window.get_key(Key::Key1) == Action::Press {
                100.0
            } else if window.get_key(Key::Key2) == Action::Press {
                500.0
            } else if window.get_key(Key::Key3) == Action::Press {
                2500.0
            } else if window.get_key(Key::Key4) == Action::Press {
                50000.0
            } else {
                5.0
            };
            sys.run(Duration::from_secs(3600 * 24), 60.0 * 60.0 * 24.0 * 365.25 * 0.00005 * speed);
        }
    }
}
