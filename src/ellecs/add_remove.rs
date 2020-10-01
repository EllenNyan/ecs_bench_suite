use ellecs::entities::Entity;
use ellecs::world::World;

#[derive(Copy, Clone)]
struct A(f32);
#[derive(Copy, Clone)]
struct B(f32);

pub struct Benchmark(World, Box<[Entity]>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut entities = Vec::with_capacity(10000);

        for _ in 0..10_000 {
            entities.push(world.spawn((A(1.),)));
        }

        Benchmark(world, entities.into_boxed_slice())
    }

    pub fn run(&mut self) {
        for &entity in self.1.iter() {
            self.0.add_component(entity, B(1.));
        }
        for &entity in self.1.iter() {
            self.0.remove_component::<B>(entity);
        }
    }
}
