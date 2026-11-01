fn sample_render_entity(game: &Game<'_>, block: &str, draw: &mut BlockEntityDraw) {
    if block == super::machine_sample::MACHINE {
        eprintln!("TEST_5707_SUBMIT");
        draw.translate([0.0, 1.0, 0.0]);
        game.native_draw_block(draw, "minecraft:gold_block", &[]);
        draw.keep();
    }
}
