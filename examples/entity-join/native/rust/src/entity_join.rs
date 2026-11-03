// Observe joins without changing the cancellation decision.
fn entity_join(
    dimension: &str,
    entity: u32,
    kind: &str,
    pos: [f64; 3],
    loaded_from_disk: bool,
    canceled: &mut bool,
) {
    let _ = canceled;
    eprintln!(
        "entity-join-example dimension={dimension} entity={entity} kind={kind} pos={pos:?} loaded={loaded_from_disk}"
    );
}
