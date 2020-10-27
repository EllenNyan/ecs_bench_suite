use ellecs::entities::Entity;
use ellecs::spawn;
use ellecs::world::World;

pub struct A(f32);

pub struct Benchmark(World, Box<[Entity]>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut entities = Vec::with_capacity(10_000);

        for _ in 0..10_000 {
            let entity = spawn!(&mut world, A(10.0));
            entities.push(entity);
        }

        Benchmark(world, entities.into_boxed_slice())
    }

    pub fn run(&mut self) {
        for &entity in self.1.iter() {
            let a = self.0.get_component_mut::<A>(entity).unwrap();
            criterion::black_box(a);
        }
    }
}
