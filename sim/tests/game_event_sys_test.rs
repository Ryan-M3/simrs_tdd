use sim::game_event::{GameEventSys, Trigger, Resolver};

struct AlwaysFireA;
impl Trigger for AlwaysFireA { fn should_fire(&mut self) -> bool { true } }
struct AlwaysFireB;
impl Trigger for AlwaysFireB { fn should_fire(&mut self) -> bool { true } }
struct NeverFire;
impl Trigger for NeverFire { fn should_fire(&mut self) -> bool { false } }

#[derive(Clone)]
struct Count(std::rc::Rc<std::cell::RefCell<usize>>);
struct IncA(Count);
impl Resolver for IncA { fn resolve(&mut self) { *self.0.0.borrow_mut() += 1; } }
struct IncB(Count);
impl Resolver for IncB { fn resolve(&mut self) { *self.0.0.borrow_mut() += 1; } }

#[test]
fn all_triggers_must_fire_and_all_resolvers_run() {
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = Count(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFireA)
        .with_trigger(AlwaysFireB)
        .with_resolver(IncA(c.clone()))
        .with_resolver(IncB(c.clone()));
    let fired = sys.run_once();
    assert!(fired);
    assert_eq!(*count.borrow(), 2);
}

#[test]
fn any_non_firing_trigger_prevents_resolvers() {
    let count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = Count(count.clone());
    let mut sys = GameEventSys::new()
        .with_trigger(AlwaysFireA)
        .with_trigger(NeverFire)
        .with_resolver(IncA(c.clone()))
        .with_resolver(IncB(c.clone()));
    let fired = sys.run_once();
    assert!(!fired);
    assert_eq!(*count.borrow(), 0);
}
