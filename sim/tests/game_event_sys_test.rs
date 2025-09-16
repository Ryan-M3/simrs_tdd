use sim::game_event::{GameEventSys, Trigger, Resolver};

struct DummyTrigger;
impl Trigger for DummyTrigger {}

struct DummyResolver;
impl Resolver for DummyResolver {}

#[test]
fn can_construct_game_event_sys() {
    let _ = GameEventSys::new();
}

#[test]
fn builder_syntax_accepts_trigger() {
    let _ = GameEventSys::new().with_trigger(DummyTrigger);
}

#[test]
fn builder_accepts_frequency() {
    use core::time::Duration;
    let _ = GameEventSys::new()
        .with_trigger(DummyTrigger)
        .with_freq(Duration::from_secs(300));
}

#[test]
fn builder_accepts_resolver() {
    use core::time::Duration;
    let _ = GameEventSys::new()
        .with_trigger(DummyTrigger)
        .with_freq(Duration::from_secs(300))
        .with_resolver(DummyResolver);
}

struct AlwaysFire;
impl Trigger for AlwaysFire {
    fn should_fire(&mut self) -> bool {
        true
    }
}

struct NeverFire;
impl Trigger for NeverFire {
    fn should_fire(&mut self) -> bool {
        false
    }
}

struct CounterResolver(std::rc::Rc<std::cell::RefCell<usize>>);
impl Resolver for CounterResolver {
    fn resolve(&mut self) {
        *self.0.borrow_mut() += 1;
    }
}

#[test]
fn run_once_calls_resolver_when_trigger_fires() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_freq(Duration::from_secs(60))
        .with_resolver(resolver);
    let fired = sys.run_once();
    assert_eq!(*count.borrow(), 1);
    assert!(fired);
}

#[test]
fn run_once_returns_false_when_trigger_does_not_fire() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(NeverFire)
        .with_freq(Duration::from_secs(60))
        .with_resolver(resolver);
    let fired = sys.run_once();
    assert_eq!(*count.borrow(), 0);
    assert!(!fired);
}

#[test]
fn tick_under_freq_does_not_fire_even_if_trigger_would() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_freq(Duration::from_secs(60))
        .with_resolver(resolver);
    let fired = sys.tick(Duration::from_secs(59));
    assert_eq!(*count.borrow(), 0);
    assert!(!fired);
}

#[test]
fn tick_at_or_over_freq_fires_when_trigger_fires() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_freq(Duration::from_secs(60))
        .with_resolver(resolver);
    let fired = sys.tick(Duration::from_secs(60));
    assert_eq!(*count.borrow(), 1);
    assert!(fired);
}

#[test]
fn tick_accumulates_time_and_fires_when_sum_reaches_freq() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_freq(Duration::from_secs(60))
        .with_resolver(resolver);

    let fired1 = sys.tick(Duration::from_secs(30));
    assert!(!fired1);
    assert_eq!(*count.borrow(), 0);

    let fired2 = sys.tick(Duration::from_secs(30));
    assert!(fired2);
    assert_eq!(*count.borrow(), 1);
}

#[test]
fn builder_accepts_poisson_rate_and_zero_rate_never_fires() {
    use core::time::Duration;
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_resolver(resolver)
        .with_poisson_rate(0.0);
    let fired = sys.tick(Duration::from_secs(3600));
    assert!(!fired);
    assert_eq!(*count.borrow(), 0);
}

use sim::game_event::PoissonSampler;

struct DummySampler;
impl PoissonSampler for DummySampler {
    fn sample(&mut self, _dt: core::time::Duration, _rate: f64) -> u64 {
        0
    }
}

#[test]
fn builder_accepts_poisson_sampler() {
    struct DummyTrigger;
    impl Trigger for DummyTrigger {}

    struct DummyResolver;
    impl Resolver for DummyResolver {}

    let _ = GameEventSys::new()
        .with_trigger(DummyTrigger)
        .with_resolver(DummyResolver)
        .with_poisson_rate(1.0)
        .with_poisson_sampler(DummySampler);
}

#[test]
fn poisson_sampler_zero_prevents_firing_even_if_trigger_would_fire() {
    use core::time::Duration;

    struct ZeroSampler;
    impl sim::game_event::PoissonSampler for ZeroSampler {
        fn sample(&mut self, _dt: Duration, _rate: f64) -> u64 {
            0
        }
    }

    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFire)
        .with_resolver(resolver)
        .with_poisson_rate(1.0)
        .with_poisson_sampler(ZeroSampler);

    let fired = sys.tick(Duration::from_secs(1));
    assert!(!fired);
    assert_eq!(*count.borrow(), 0);
}

#[test]
fn builder_accepts_proximity_trigger_with_graph() {
    use sim::graph::Graph;
    use sim::game_event::{GameEventSys, Proximity};

    let graph = Graph::new();
    let _ = GameEventSys::new().with_trigger(Proximity::new(graph));
}

#[test]
fn proximity_triggers_when_adjacent() {
    use sim::graph::Graph;
    use sim::game_event::Proximity;
    use core::time::Duration;

    let mut g = Graph::new();
    g.add_edge(1usize, 2usize, 1usize);

    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let resolver = CounterResolver(count.clone());

    let mut sys = GameEventSys::new()
        .with_trigger(Proximity::new(g).with_pair(1usize, 2usize))
        .with_resolver(resolver)
        .with_freq(Duration::from_secs(0)); // ensure time gating doesn't block

    let fired = sys.run_once();
    assert!(fired);
    assert_eq!(*count.borrow(), 1);
}
