fn sample_draw(game: &Game<'_>, block: &str, data: &[i32], batch: &mut HudBatch, origin: [f32; 2]) {
    if block == super::example_mod::MACHINE {
        let progress = data
            .first()
            .copied()
            .unwrap_or_default()
            .clamp(0, super::example_mod::DURATION);
        game.native_fill(
            batch,
            [origin[0] + 72.0, origin[1] + 40.0, 32.0, 6.0],
            0xff555555,
        );
        game.native_fill(
            batch,
            [
                origin[0] + 72.0,
                origin[1] + 40.0,
                32.0 * progress as f32 / super::example_mod::DURATION as f32,
                6.0,
            ],
            0xff55bb55,
        );
    }
}
