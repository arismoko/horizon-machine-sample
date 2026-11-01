fn tell_test(game: &mut Game<'_>) {
    if let Some(player) = game.native_acting_player() {
        game.native_tell(player.uuid, horizon_client::text::Text::literal("Native tell reached acting player"));
    }
}
