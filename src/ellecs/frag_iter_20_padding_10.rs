use ellecs::world::World;

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

            fn spawn_entity<T: 'static>(world: &mut World, data: T) {
                world.spawn((data, $($y(2.),)* Data(1.)));
            }
        };
    }

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Benchmark {
        let mut world = World::new();
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
        Benchmark(world)
    }

    pub fn run(&mut self) {
        self.0.query::<(&mut Data,)>().borrow().for_each(|(data,)| {
            data.0 *= 2.;
        });
    }
}
