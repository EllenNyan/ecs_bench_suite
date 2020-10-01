use legion::*;
use query::Query;
use storage::PackOptions;
pub struct Data(f32);

macro_rules! setup {
    ($world:ident, (bloat: ($($y:ident,)*)), ($($x:ident),*)) => {
        $(
            pub struct $x(f32);
        )*

        $(
            pub struct $y(f32);
        )*

        $(
            for _ in 0..20 {
                spawn_entity(&mut $world, $x);
            }
        )*

        fn spawn_entity<T: Send + Sync + 'static>(world: &mut World, data: T) {
            world.extend(vec![(data, $($y(2.),)* Data(1.))])[0];
        }
    };
}

pub struct Benchmark(World, Query<Write<Data>>);

impl Benchmark {
    pub fn new() -> Benchmark {
        let mut world = World::default();
        setup!(
            world,
            (bloat:
                (
                    Bloat1,
                    Bloat2,
                    Bloat3,
                    Bloat4,
                    Bloat5,
                    Bloat6,
                    Bloat7,
                    Bloat8,
                    Bloat9,
                    Bloat10,
                )),
            (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
        );
        world.pack(PackOptions::force());

        let query = Write::<Data>::query();

        Self(world, query)
    }

    pub fn run(&mut self) {
        self.1.for_each_mut(&mut self.0, |data| {
            data.0 *= 2.0;
        });
    }
}
