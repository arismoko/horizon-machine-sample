fn sample_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if entity.is_none() && (block == horizon_example_mod::MACHINE) {
        *entity = Some((block.to_owned(), Compound::new()));
    }
}
