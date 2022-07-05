use shipyard::{self as ecs, IntoIter, IntoWithId, Get};
use std::{
    ops::Deref,
    time::Duration,
};
use vek::*;

pub use vek::{Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Mass(pub f64);

#[derive(Clone, Debug)]
pub struct Pos(pub Vec3<f64>);

#[derive(Clone, Debug)]
pub struct Vel(pub Vec3<f64>);

pub struct Acc(pub Vec3<f64>);

#[derive(Clone, Debug)]
pub struct Radius(pub f64);

#[derive(Copy, Clone, Debug)]
pub struct Id(ecs::EntityId);

struct Dt(f64);

// Helper constructors

pub trait Constructor {
    fn add(self, sys: &mut System) -> Id;
}

pub struct SimpleBody {
    pub pos: Pos,
    pub vel: Vel,
    pub mass: Mass,
    pub radius: Radius,
}

impl Constructor for SimpleBody {
    fn add(self, sys: &mut System) -> Id {
        Id(sys.ecs.add_entity((
            self.pos,
            self.vel,
            self.mass,
            self.radius,
            Acc(Vec3::zero()),
        )))
    }
}

pub struct System {
    ecs: ecs::World,
}

impl Default for System {
    fn default() -> Self { Self::new() }
}

impl System {
    pub fn new() -> Self {
        let mut ecs = ecs::World::default();
        ecs.add_unique(Dt(0.0));
        Self { ecs }
    }

    pub fn add<C: Constructor>(&mut self, con: C) -> Id { con.add(self) }

    pub fn run_tick(&mut self, dt: f64) {
        self.ecs.borrow::<ecs::UniqueViewMut<Dt>>().unwrap().0 = dt;
        self.ecs.run(tick);
    }

    pub fn get<C: Clone + Send + Sync + 'static>(&self, id: Id) -> Option<C> {
        self.ecs
            .borrow::<ecs::View<C>>()
            .ok()
            .and_then(|c| c.get(id.0).ok().cloned())
    }

    pub fn run(&mut self, dt: Duration, time: f64) {
        let dt = dt.as_secs_f64();
        let dt_time = time / dt;

        self.ecs.run(init);

        for _ in 0..dt_time.floor() as usize {
            self.run_tick(dt);
        }
        self.run_tick(dt * dt_time.fract());
    }
}

const G: f64 = 0.00000000006674;

fn init(
    mut pos: ecs::ViewMut<Pos>,
    mut acc: ecs::ViewMut<Acc>,
    mass: ecs::View<Mass>,
    mut vel: ecs::ViewMut<Vel>,
) {
    update_acc(&pos, &mass, &mut acc);
}

fn tick(
    dt: ecs::UniqueView<Dt>,
    mut pos: ecs::ViewMut<Pos>,
    mut acc: ecs::ViewMut<Acc>,
    mass: ecs::View<Mass>,
    mut vel: ecs::ViewMut<Vel>,
) {
    for (mut vel, acc) in (&mut vel, &acc).iter() {
        vel.0 += acc.0 * dt.0 / 2.0;
    }

    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.0 += vel.0 * dt.0;
    }

    update_acc(&pos, &mass, &mut acc);

    for (mut vel, acc) in (&mut vel, &acc).iter() {
        vel.0 += acc.0 * dt.0 / 2.0;
    }
}

fn update_acc(
    pos: &ecs::ViewMut<Pos>,
    mass: &ecs::View<Mass>,
    acc: &mut ecs::ViewMut<Acc>,
) {
    for (e0, (pos0, mass0, mut acc0)) in (pos, mass, acc).iter().with_id() {
        let mut net_force = Vec3::<f64>::zero();
        for (_, (pos1, mass1)) in (pos, mass)
            .iter()
            .with_id()
            .filter(|(e1, _)| e0 != *e1)
        {
            let diff = pos1.0 - pos0.0;
            let dist = diff.magnitude();
            let dir = diff / dist;

            let force = G * (mass0.0 * mass1.0) / dist.powi(2);
            net_force += force * dir;
        }

        acc0.0 = net_force / mass0.0;
    }
}
