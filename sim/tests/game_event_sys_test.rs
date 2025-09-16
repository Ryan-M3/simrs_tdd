use sim::game_event::GameEventSys;

struct DummyTrigger;

#[test]
fn can_construct_game_event_sys() {
    let _ = GameEventSys::new();
}

#[test]
fn builder_syntax_accepts_trigger() {
    let _ = GameEventSys::new().with_trigger(DummyTrigger);
}
