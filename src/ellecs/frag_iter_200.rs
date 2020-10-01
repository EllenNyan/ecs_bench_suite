use ellecs::world::World;

pub struct Data(f32);

macro_rules! setup {
    ($world:ident, $($x:ident),*) => {
        $(
            pub struct $x(f32);
        )*

        $(
            for _ in 0..200 {
                $world.spawn(($x(0.), Data(1.)));
            }
        )*
    };
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        setup!(world, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
        Benchmark(world)
    }

    pub fn run(&mut self) {
        self.0.query::<(&mut Data,)>().borrow().for_each(|(data,)| {
            data.0 *= 2.;
        });
    }
}
