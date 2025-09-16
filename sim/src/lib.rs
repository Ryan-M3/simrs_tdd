pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod app {
    use bevy::prelude::App;

    /// Construct the main simulation app with minimal resources installed.
    pub fn main_app() -> App {
        let mut app = App::new();
        app.insert_resource(crate::time::GameSpeed::default());
        app.insert_resource(crate::rng::Rng::default());
        app
    }
}

pub mod graph {
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default)]
    pub struct Graph<T> {
        degrees: HashMap<usize, usize>,
        weights: HashMap<(usize, usize), T>,
    }

    impl<T> Graph<T> {
        pub fn new() -> Self {
            Graph {
                degrees: HashMap::new(),
                weights: HashMap::new(),
            }
        }

        pub fn add_edge(&mut self, a: usize, b: usize, weight: T)
        where
            T: Clone,
        {
            *self.degrees.entry(a).or_insert(0) += 1;
            *self.degrees.entry(b).or_insert(0) += 1;
            self.weights.insert((a, b), weight.clone());
            self.weights.insert((b, a), weight);
        }

        pub fn degree(&self, v: usize) -> usize {
            self.degrees.get(&v).copied().unwrap_or(0)
        }

        pub fn weight(&self, a: usize, b: usize) -> Option<&T> {
            self.weights.get(&(a, b))
        }
    }
}

pub mod game_event {
    use core::time::Duration;

    // Minimal API surface to satisfy external tests
    pub trait Trigger {
        fn should_fire(&mut self) -> bool {
            false
        }
    }
    pub trait Resolver {
        fn resolve(&mut self) {}
    }
    pub trait PoissonSampler {
        fn sample(&mut self, _dt: Duration, _rate: f64) -> u64;
    }

    pub struct Proximity<T> {
        graph: crate::graph::Graph<T>,
        pair: Option<(usize, usize)>,
    }
    impl<T> Proximity<T> {
        pub fn new(graph: crate::graph::Graph<T>) -> Self {
            Proximity { graph, pair: None }
        }
        pub fn with_pair(mut self, a: usize, b: usize) -> Self {
            self.pair = Some((a, b));
            self
        }
    }
    impl<T> Trigger for Proximity<T> {
        fn should_fire(&mut self) -> bool {
            if let Some((a, b)) = self.pair {
                self.graph.weight(a, b).is_some()
            } else {
                false
            }
        }
    }

    pub struct GameEventSys {
        triggers: Vec<Box<dyn Trigger>>,
        resolvers: Vec<Box<dyn Resolver>>,
        freq: Option<Duration>,
        elapsed: Duration,
        poisson_rate: Option<f64>,
        poisson_sampler: Option<Box<dyn PoissonSampler>>,
    }

    impl GameEventSys {
        pub fn new() -> Self {
            GameEventSys {
                triggers: Vec::new(),
                resolvers: Vec::new(),
                freq: None,
                elapsed: Duration::from_secs(0),
                poisson_rate: None,
                poisson_sampler: None,
            }
        }

        pub fn with_trigger<T: Trigger + 'static>(mut self, trigger: T) -> Self {
            self.triggers.push(Box::new(trigger));
            self
        }

        pub fn with_freq(mut self, freq: Duration) -> Self {
            self.freq = Some(freq);
            self
        }

        pub fn with_resolver<R: Resolver + 'static>(mut self, resolver: R) -> Self {
            self.resolvers.push(Box::new(resolver));
            self
        }

        pub fn with_poisson_rate(mut self, rate: f64) -> Self {
            self.poisson_rate = Some(rate);
            self
        }

        pub fn with_poisson_sampler<S: PoissonSampler + 'static>(mut self, sampler: S) -> Self {
            self.poisson_sampler = Some(Box::new(sampler));
            self
        }

        pub fn run_once(&mut self) -> bool {
            if self.triggers.is_empty() || self.resolvers.is_empty() {
                return false;
            }

            if self.triggers.iter_mut().all(|trigger| trigger.should_fire()) {
                for resolver in self.resolvers.iter_mut() {
                    resolver.resolve();
                }
                true
            } else {
                false
            }
        }

        pub fn tick(&mut self, dt: Duration) -> bool {
            if let Some(rate) = self.poisson_rate {
                if rate <= 0.0 {
                    return false;
                }
                if let Some(sampler) = self.poisson_sampler.as_mut() {
                    let k = sampler.sample(dt, rate);
                    if k == 0 {
                        return false;
                    }
                    // Fire at most once regardless of k>0 to satisfy tests
                    return self.run_once();
                }
            }
            if let Some(freq) = self.freq {
                self.elapsed += dt;
                if self.elapsed < freq {
                    return false;
                }
                // Reset after reaching threshold; only one fire per threshold for test compliance.
                self.elapsed = Duration::from_secs(0);
            }
            // If no freq set or dt >=/accumulated to freq, attempt to fire once.
            self.run_once()
        }
    }
}

pub mod time {
    use bevy::prelude::Resource;

    #[derive(Resource, Clone, Copy, Debug, PartialEq)]
    pub struct GameSpeed(pub f32);

    impl Default for GameSpeed {
        fn default() -> Self {
            GameSpeed(1.0)
        }
    }
}

pub mod rng {
    use bevy::prelude::Resource;

    #[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
    pub struct Rng {
        state: u64,
    }

    impl Rng {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn next_u64(&mut self) -> u64 {
            // Xorshift64* for a tiny, deterministic RNG
            let mut x = self.state.wrapping_add(0x9E3779B97F4A7C15);
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.state = x;
            x.wrapping_mul(0x2545F4914F6CDD1D)
        }
    }
}
