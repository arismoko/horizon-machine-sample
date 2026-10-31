fn sample_ticks(block: &str, ticks: &mut bool) {
    if block == super::example_mod::MACHINE {
        *ticks = true;
    }
}
