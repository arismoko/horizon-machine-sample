fn sample_entity_join(dimension: &str, entity: u32, kind: &str, pos: [f64; 3], loaded_from_disk: bool, canceled: &mut bool) {
    *canceled = pos[1] > 200.0;
    eprintln!("PR5167 join dimension={dimension} entity={entity} kind={kind} pos={pos:?} loaded={loaded_from_disk} canceled={canceled}");
}
