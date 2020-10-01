use cgmath::*;
use ellecs::world::World;

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        for _ in 0..1_000 {
            world.spawn((Matrix4::<f32>::from_angle_x(Rad(1.2)),));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        let query = self.0.query::<(&mut Matrix4<f32>,)>();
        query.borrow().for_each(|(matrix,)| {
            for _ in 0..100 {
                matrix.invert().unwrap();
            }
        });
    }
}
