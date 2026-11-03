fn sample_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    use horizon_world::nbt::Tag;
    if entity.id == horizon_example_mod::MACHINE {
        use horizon_client::mod_api::{Host, HostTransaction, ITEMS, ItemResource, ServerHost};
        let dimension = Host::dimension(game);
        let host: &mut dyn ServerHost = game;
        horizon_example_mod::fill_empty(&mut entity.items[0], || {
            let source = host.level(dimension)?.capability(
                host,
                &ITEMS,
                [pos[0] - 1, pos[1], pos[2]],
                Some(Direction::East),
            )?;
            host.transaction(|host, tx| {
                let resource = source.resource(host, 0).ok_or(())?;
                if source.extract(host, tx, 0, &resource, 1) == 1 {
                    Ok(resource.to_stack(1))
                } else {
                    Err(())
                }
            }).ok()
        });
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
        let (progress, done) = horizon_example_mod::advance(progress, result.is_some());
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
        let dimension = Host::dimension(game);
        let host: &mut dyn ServerHost = game;
        let pipe = host
            .level(dimension)
            .and_then(|level| {
                level.block(host, [pos[0] + 1, pos[1], pos[2]])
            });
        if pipe.as_deref() == Some(horizon_example_mod::PIPE) {
            if let Some(output) = &mut entity.items[1] {
                let resource = ItemResource::of(output);
                let dimension = Host::dimension(game);
                let host: &mut dyn ServerHost = game;
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
