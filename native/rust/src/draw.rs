fn sample_draw(game: &Game<'_>, block: &str, data: &[i32], batch: &mut HudBatch, origin: [f32; 2]) {
    use horizon_client::mod_api::ClientHost;
    if block == horizon_example_mod::MACHINE {
        ClientHost::fill(
            game,
            batch,
            [origin[0] + 66.0, origin[1] + 55.0, 44.0, 16.0],
            0xff555555,
        );
        ClientHost::text(
            game,
            batch,
            "Deposit",
            [origin[0] + 68.0, origin[1] + 59.0],
            0xffffff,
            false,
        );
        let progress = data
            .first()
            .copied()
            .unwrap_or_default()
            .clamp(0, horizon_example_mod::DURATION);
        ClientHost::fill(
            game,
            batch,
            [origin[0] + 72.0, origin[1] + 40.0, 32.0, 6.0],
            0xff555555,
        );
        ClientHost::fill(
            game,
            batch,
            [
                origin[0] + 72.0,
                origin[1] + 40.0,
                32.0 * progress as f32 / horizon_example_mod::DURATION as f32,
                6.0,
            ],
            0xff55bb55,
        );
    }
}
