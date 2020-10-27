use cgmath::*;
use ellecs::spawn;
use ellecs::world::World;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
struct Position(Vector3<f32>);
#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);
#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        for _ in 0..10_000 {
            spawn!(
                &mut world,
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            );
        }

        Benchmark(world)
    }

    pub fn run(&mut self) {
        self.0
            .query::<(&mut Position, &mut Velocity)>()
            .borrow()
            .for_each(|(pos, vel)| {
                pos.0 += vel.0;
            });
    }
}
