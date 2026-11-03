fn sample_ticks(block: &str, ticks: &mut bool) {
    if block == horizon_example_mod::MACHINE {
        *ticks = true;
    }
}
