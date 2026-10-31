fn sample_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if entity.is_none() && (block == super::example_mod::MACHINE) {
        *entity = Some((block.to_owned(), Compound::new()));
    }
}
