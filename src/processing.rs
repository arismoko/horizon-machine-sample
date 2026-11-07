//! Reload-owned definitions; inventory and world answers are never cached.
use mod_api::*;
use std::sync::RwLock;

struct ProcessingRecipe {
    ingredient: Ingredient,
    result: ItemStack,
}

#[derive(Default)]
pub(crate) struct ProcessingRecipes {
    published: RwLock<Vec<ProcessingRecipe>>,
}

impl ProcessingRecipes {
    pub(crate) fn clear(&self) {
        self.published.write().unwrap().clear();
    }

    pub(crate) fn publish(&self, host: &dyn Host) {
        // Build completely before replacing, including when all recipes vanished.
        let next = host
            .recipes_of_type("machine_sample:processing")
            .iter()
            .filter_map(|recipe| {
                Some(ProcessingRecipe {
                    ingredient: host.recipe_ingredient(&recipe.json["ingredient"])?,
                    result: host.recipe_result(&recipe.json["result"])?,
                })
            })
            .collect();
        *self.published.write().unwrap() = next;
    }

    pub(crate) fn result(&self, host: &dyn Host, input: &ItemStack) -> Option<ItemStack> {
        let table = self.published.read().unwrap();
        table
            .iter()
            .find(|recipe| recipe.ingredient.test(host, input))
            .map(|recipe| recipe.result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::{__private::game::BackendRefOf, prelude::ItemsHost};
    use std::{cell::Cell, sync::Arc};

    struct Data {
        recipes: Arc<[Recipe]>,
        accepts: Cell<bool>,
    }
    impl Host for Data {
        fn backend(&self) -> Option<BackendRef<'_>> {
            Some(BackendRef::of(self))
        }
    }
    impl mod_api::prelude::RecipesHost for Data {
        fn recipes_of_type(&self, _: &str) -> Arc<[Recipe]> {
            self.recipes.clone()
        }
        fn recipe_ingredient(&self, json: &serde_json::Value) -> Option<Ingredient> {
            Some(Ingredient::of(vec![json.as_str()?.to_owned()]))
        }
        fn recipe_result(&self, json: &serde_json::Value) -> Option<ItemStack> {
            ItemStack::new(key(json.as_u64()? as u32), 1)
        }
    }
    mod_api::impl_host_areas!(Data, except(recipes));
    impl Backend for Data {}
    impl mod_api::prelude::RecipesBackend for Data {
        fn recipe_ingredient_test(&self, ingredient: &Ingredient, input: &ItemStack) -> bool {
            self.accepts.get() && ingredient.items.contains(&input.count().to_string())
        }
    }
    mod_api::impl_backend_areas!(Data, except(recipes));

    fn key(n: u32) -> ItemKey {
        let mut host = mod_api::testing::FakeHost::new();
        host.registries
            .resolve_items((1..=4).map(|i| Identifier::parse(&format!("test:item_{i}")).unwrap()))
            .unwrap();
        host.item_key(&Identifier::parse(&format!("test:item_{n}")).unwrap())
            .unwrap()
    }

    fn data(results: &[u32]) -> Data {
        Data {
            recipes: results
                .iter()
                .map(|result| {
                    Recipe::new(
                        "test:r",
                        "machine_sample:processing",
                        Arc::new(serde_json::json!({"ingredient": "1", "result": result})),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
            accepts: Cell::new(true),
        }
    }

    #[test]
    fn publication_replaces_order_overrides_and_removals() {
        let cache = ProcessingRecipes::default();
        let input = ItemStack::new(key(1), 1).unwrap();
        assert!(cache.result(&data(&[]), &input).is_none());
        let original = data(&[2, 3]);
        cache.publish(&original);
        assert_eq!(cache.result(&original, &input).unwrap().item(), key(2));
        let override_ = data(&[4]);
        cache.publish(&override_);
        assert_eq!(cache.result(&override_, &input).unwrap().item(), key(4));
        cache.publish(&data(&[]));
        assert!(cache.result(&original, &input).is_none());
    }

    #[test]
    fn stopped_and_next_start_cannot_reuse_world_definitions() {
        let sample = crate::Sample::default();
        let source = data(&[2]);
        let input = ItemStack::new(key(1), 1).unwrap();
        let mut host = mod_api::testing::FakeHost::new();
        sample.recipes.publish(&source);
        sample.server_stopped(&mut host, &mut ServerStoppedEvent { dedicated: false });
        assert!(sample.recipes.result(&source, &input).is_none());
        sample.recipes.publish(&source);
        sample.server_about_to_start(&mut host, &mut ServerAboutToStartEvent { dedicated: true });
        assert!(sample.recipes.result(&source, &input).is_none());
    }

    #[test]
    fn live_input_and_world_answers_are_not_cached() {
        let cache = ProcessingRecipes::default();
        let host = data(&[2]);
        cache.publish(&host);
        let input = ItemStack::new(key(1), 1).unwrap();
        assert!(cache.result(&host, &input).is_some());
        let changed = ItemStack::new(key(1), 2).unwrap();
        assert!(cache.result(&host, &changed).is_none());
        host.accepts.set(false);
        assert!(cache.result(&host, &input).is_none());
        host.accepts.set(true);
        assert!(cache.result(&host, &input).is_some());
        cache.clear();
        assert!(cache.result(&host, &input).is_none());
    }
}
