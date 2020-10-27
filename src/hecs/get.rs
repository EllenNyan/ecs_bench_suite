use hecs::*;

#[derive(Copy, Clone)]
struct A(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let entities = world
            .spawn_batch((0..10_000).map(|_| (A(0.0),)))
            .collect::<Vec<_>>();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for &entity in &self.1 {
            criterion::black_box(self.0.get::<A>(entity).unwrap());
        }
    }
}
