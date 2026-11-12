fn head(game: &mut Game<'_>) {
    crate::game::autosave::sample_telemetry::notice(
        *game.__horizon_mixins_machine_sample_autosave(),
    );
}
