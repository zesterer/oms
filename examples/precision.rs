use oms::{System, Id, SimpleBody, Mass, Pos, Vel, Radius, Vec3};
use std::time::Duration;
use plotters::prelude::*;

fn simulate(step: Duration) -> Vec3<f64> {
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

    // println!("Simulating 1000 years..");
    sys.run(step, 60.0 * 60.0 * 24.0 * 365.25 * 100.0);

    let sun_pos = sys.get::<Pos>(sun).unwrap().0;
    let earth_pos = sys.get::<Pos>(earth).unwrap().0;
    let moon_pos = sys.get::<Pos>(moon).unwrap().0;
    println!("Step = {:?}, Moon position: {:?}", step, moon_pos);

    moon_pos
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let correct = simulate(Duration::from_secs(100));

    let mut data = Vec::new();
    for i in 1..25 {
        let step = Duration::from_secs((100.0 * 2f32.powi(i)) as u64);
        let pos = simulate(step);
        let error = correct.distance(pos);
        println!("Error for {:?} is {}", step, error);
        data.push((
            step.as_secs_f32(),
            error as f32,
        ));
    }

    let root = SVGBackend::new("target/precision.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Simulation precision over 100 years", ("sans-serif", 50).into_font())
        .margin(100)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (data.first().unwrap().0..data.last().unwrap().0).log_scale(),
            (data.first().unwrap().1..data.last().unwrap().1).log_scale(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Time step (seconds)")
        .y_desc("Inaccuracy (metres)")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            data,
            &RED,
        ))?
        .label("location inaccuracy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
