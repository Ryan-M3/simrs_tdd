#[test]
fn sim_binary_runs_successfully() {
    let exe = env!("CARGO_BIN_EXE_sim");
    let status = std::process::Command::new(exe).status().expect("failed to run sim binary");
    assert!(status.success());
}
