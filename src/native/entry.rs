fn head() {
    static FIRST: std::sync::Once = std::sync::Once::new();
    FIRST.call_once(|| eprintln!("machine_sample: native block-entity pass entered"));
}
