fn sample_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    use horizon_world::nbt::Tag;
    if entity.id == super::machine_sample::MACHINE {
        if !entity.nbt.contains_key(&"random_probe".into()) {
            use horizon_core::random::RandomSource;
            let draw = game
                .native_level_random("minecraft:overworld")
                .map(|rng| rng.next_int());
            let invalid = game.native_level_random("example:invalid").is_none();
            eprintln!("PR5146 random overworld={draw:?} invalid_none={invalid}");
            entity.nbt.insert("random_probe".into(), Tag::Int(1));
        }
        let result = entity
            .items
            .first()
            .and_then(Option::as_ref)
            .and_then(|input| {
                horizon_client::mod_api::Host::recipes_of_type(game, "machine_sample:processing")
                    .iter()
                    .find_map(|recipe| {
                        let ingredient = horizon_client::mod_api::Host::recipe_ingredient(
                            game,
                            &recipe.json["ingredient"],
                        )?;
                        if !ingredient.test(input) {
                            return None;
                        }
                        horizon_client::mod_api::Host::recipe_result(game, &recipe.json["result"])
                    })
            });
        let result = result.filter(|result| {
            let limit = game
                .items()
                .get(&result.item)
                .map_or(0, |item| item.max_stack);
            match entity.items.get(1).and_then(Option::as_ref) {
                None => result.count <= limit,
                Some(output) => {
                    output.item == result.item
                        && output.damage == result.damage
                        && output.extra == result.extra
                        && output.count + result.count <= limit
                }
            }
        });
        let progress = match entity.nbt.get(&"progress".into()) {
            Some(Tag::Int(n)) => *n,
            _ => 0,
        };
        let (progress, done) = super::machine_sample::advance(progress, result.is_some());
        entity.nbt.insert("progress".into(), Tag::Int(progress));
        entity.data = vec![progress];
        if done {
            let input = entity.items[0].as_mut().unwrap();
            input.count -= 1;
            if input.count == 0 {
                entity.items[0] = None;
            }
            let result = result.unwrap();
            if let Some(output) = &mut entity.items[1] {
                output.count += result.count;
            } else {
                entity.items[1] = Some(result);
            }
        }
        let dimension = horizon_client::mod_api::Host::dimension(game);
        let host: &mut dyn horizon_client::mod_api::Host = game;
        let pipe = host
            .level(dimension)
            .and_then(|level| {
                level.block(host, [pos[0] + 1, pos[1], pos[2]])
            });
        if pipe.as_deref() == Some(super::machine_sample::PIPE) {
            if let Some(output) = &mut entity.items[1] {
                use horizon_client::mod_api::{Host, ITEMS, ItemResource};
                let resource = ItemResource::of(output);
                let dimension = Host::dimension(game);
                let host: &mut dyn Host = game;
                let inserted = host
                    .level(dimension)
                    .and_then(|level| {
                        level.capability(
                            host,
                            &ITEMS,
                            [pos[0] + 2, pos[1], pos[2]],
                            Some(Direction::West),
                        )
                    })
                    .is_some_and(|target| {
                        host.transaction(|host, tx| {
                            if target.insert_any(host, tx, &resource, 1) == 1 {
                                Ok(())
                            } else {
                                Err(())
                            }
                        })
                        .is_ok()
                    });
                if inserted {
                    output.count -= 1;
                    if output.count == 0 {
                        entity.items[1] = None;
                    }
                }
            }
        }
    }
}
