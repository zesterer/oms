use oms::{System, Id, SimpleBody, Mass, Pos, Vel, Radius, Vec3};
use std::{
    time::Duration,
    collections::VecDeque,
};
use kiss3d::{event::{Key, Action}, window::Window, light::Light, camera::ArcBall};
use rand::{thread_rng, Rng};

const SCALE: f64 = 0.000_000_01;

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
    let phobos = body(
        Some(mars),
        9_234_420.0, // dist
        2_138.0, // vel
        10_659_000_000_000_000.0, // mass
        11_266.7, // radius
    );
    let deimos = body(
        Some(mars),
        23_455_500.0, // dist
        1_351.3, // vel
        1_476_200_000_000_000.0, // mass
        6_200.0, // radius
    );
    let jupiter = body(
        Some(sun),
        au * 5.4570, // dist
        13_070.0, // vel
        1_898_200_000_000_000_000_000_000_000.0, // mass
        71_492_000.0, // radius
    );

    let mut bodies = vec![
        ("Sun", sun, [1.0, 0.8, 0.0]),
        ("Mercury", mercury, [0.8, 0.8, 0.0]),
        ("Venus", venus, [1.0, 0.9, 0.5]),
        ("Earth", earth, [0.0, 0.7, 1.0]),
        ("Moon", moon, [0.5, 0.5, 0.6]),
        ("Mars", mars, [1.0, 0.5, 0.3]),
        ("Phobos", phobos, [1.0, 0.65, 0.5]),
        ("Deimos", deimos, [0.7, 0.7, 0.65]),
        ("Jupiter", jupiter, [1.0, 0.3, 0.1]),
    ];

    // sys.run(Duration::from_secs(3600 * 24), 60.0 * 60.0 * 24.0 * 365.25 * 1000.0);

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
        .iter()
        .cloned()
        .map(|(_, id, [r, g, b])| {
            let mut body = window.add_sphere((sys.get::<Radius>(id).unwrap().0 * SCALE)/*.powf(0.35)*/ as f32 /* * 15.0*/);
            body.set_color(r, g, b);
            (body, id, VecDeque::new())
        })
        .collect::<Vec<_>>();

    let mut camera = ArcBall::new_with_frustrum(
        0.5,
        0.001,
        100000.0,
        [0.0, 1.0, 0.0].into(),
        [0.0; 3].into(),
    );

    let mut focus_idx = 0;
    let mut time_since_action = 0;
    while !window.should_close() {
        window.set_title(&format!("Solar System ({})", bodies[focus_idx].0));
        let frame = sys.get::<Pos>(bodies[focus_idx].1).unwrap().0;

        camera.set_at((frame * SCALE).map(|e| e as f32).into_array().into());

        for (body_shape, id, path) in &mut body_shapes {
            let pos = (sys.get::<Pos>(*id).unwrap().0 * SCALE).map(|e| e as f32).into_array();
            body_shape.set_local_translation(pos.into());

            const MAX_LEN: usize = 500;
            while path.len() > MAX_LEN { path.pop_back(); }
            path.push_front(pos);

            for (i, line) in path.make_contiguous().windows(2).enumerate() {
                window.draw_line(
                    &line[0].into(),
                    &line[1].into(),
                    &[1.0 - (i as f32 / MAX_LEN as f32); 3].into(),
                );
            }
        }

        if time_since_action > 10 {
            if window.get_key(Key::Equals) == Action::Press {
                focus_idx = (focus_idx + 1) % bodies.len();
                time_since_action = 0;
            }
            if window.get_key(Key::Minus) == Action::Press {
                focus_idx = (focus_idx + bodies.len() - 1) % bodies.len();
                time_since_action = 0;
            }
        }

        if window.get_key(Key::Space) != Action::Press {
            let speed = if window.get_key(Key::Key1) == Action::Press {
                250.0
            } else if window.get_key(Key::Key2) == Action::Press {
                250.0
            } else if window.get_key(Key::Key3) == Action::Press {
                1200.0
            } else if window.get_key(Key::Key4) == Action::Press {
                25000.0
            } else {
                1.0
            };
            sys.run(Duration::from_secs(3600), 60.0 * 60.0 * 24.0 * 365.25 * 0.00005 * speed);
        }
        time_since_action += 1;

        window.render_with_camera(&mut camera);
    }
}
