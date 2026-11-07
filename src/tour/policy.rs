//! A namespaced gamerule controlling automatic machine intake.
use mod_api::*;

pub fn register(registrar: &mut dyn Registrar) -> BoolGameRuleKey {
    registrar
        .game_rule_category("machines")
        .expect("sample rule category");
    registrar
        .game_rule_bool("automatic_input", "machine_sample:machines", true)
        .expect("unique sample gamerule")
}

pub fn automatic_input(host: &dyn ServerHost, key: BoolGameRuleKey) -> bool {
    host.level(host.dimension())
        .and_then(|level| level.game_rule_bool(host, key))
        == Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_recipe_uses_the_registered_input_tag() {
        let recipe: serde_json::Value = serde_json::from_str(include_str!(
            "../../data/machine_sample/recipe/stone_processing.json"
        ))
        .unwrap();
        let tag: serde_json::Value = serde_json::from_str(include_str!(
            "../../data/machine_sample/tags/item/processable.json"
        ))
        .unwrap();
        assert_eq!(recipe["ingredient"], "#machine_sample:processable");
        assert_eq!(tag["values"], serde_json::json!(["minecraft:cobblestone"]));
    }
}
