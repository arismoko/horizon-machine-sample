use mod_api::*;

use crate::{DEPOSIT, DURATION, PIPE, advance, deposit_count, fill_empty};

pub struct Machine {
    pub automatic_input: BoolGameRuleKey,
}

#[mod_api::horizon_part]
impl BlockPart for Machine {}

#[mod_api::horizon_part]
impl BlockEntityPart for Machine {
    fn block_entity_ticks(&self, _: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        event.ticks = true;
    }

    fn block_entity_tick(&self, host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
        let pos = event.pos;
        let entity = &mut event.entity;
        let dimension = host.dimension();
        fill_empty(&mut entity.items[0], || {
            if !crate::tour::policy::automatic_input(host, self.automatic_input) {
                return None;
            }
            let source = host.level(dimension)?.capability(
                host,
                &ITEMS,
                [pos[0] - 1, pos[1], pos[2]],
                Some(Direction::East),
            )?;
            host.transaction(|host, tx| {
                let resource = source.resource(host, 0).ok_or(())?;
                if source.extract(host, tx, 0, &resource, 1) == 1 {
                    resource.to_stack(1).ok_or(())
                } else {
                    Err(())
                }
            })
            .ok()
        });
        let result = entity
            .items
            .first()
            .and_then(Option::as_ref)
            .and_then(|input| {
                host.recipes_of_type("machine_sample:processing")
                    .iter()
                    .find_map(|recipe| {
                        let ingredient = host.recipe_ingredient(&recipe.json["ingredient"])?;
                        if !ingredient.test(&*host, input) {
                            return None;
                        }
                        host.recipe_result(&recipe.json["result"])
                    })
            });
        let result = result.filter(|result| {
            let limit = host.max_stack_size(result.item()).unwrap_or(0);
            match entity.items.get(1).and_then(Option::as_ref) {
                None => result.count() <= limit,
                Some(output) => {
                    output.is_stackable_with(result)
                        && output
                            .count()
                            .checked_add(result.count())
                            .is_some_and(|n| n <= limit)
                }
            }
        });
        let progress = match entity.nbt.get(&"progress".into()) {
            Some(Tag::Int(n)) => *n,
            _ => 0,
        };
        let duration = entity.items[0].as_ref().map_or(DURATION, |input| {
            crate::tour::reload::duration(input.item())
        });
        let (progress, done) = advance(progress, result.is_some(), duration);
        entity.nbt.insert("progress".into(), Tag::Int(progress));
        entity.data = vec![progress, duration];
        if done {
            crate::tour::energy::charge(&mut entity.nbt);
            entity.items[0] = entity.items[0].take().and_then(|input| input.shrink(1));
            let result = result.unwrap();
            if let Some(output) = &mut entity.items[1] {
                output
                    .set_count(output.count() + result.count())
                    .expect("capacity checked above");
            } else {
                entity.items[1] = Some(result);
            }
        }
        let dimension = host.dimension();
        let pipe = host
            .level(dimension)
            .and_then(|level| level.block(host, [pos[0] + 1, pos[1], pos[2]]));
        if pipe.as_deref() == Some(PIPE)
            && let Some(output) = &mut entity.items[1]
        {
            let resource = ItemResource::of(output);
            let dimension = host.dimension();
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
                entity.items[1] = entity.items[1].take().and_then(|output| output.shrink(1));
            }
        }
    }

    fn screen(&self, _: &dyn Host, _: &mut ScreenEvent<'_>) -> Option<NativeScreen> {
        Some(NativeScreen {
            width: 176.0,
            height: 166.0,
            slots: vec![Some([44.0, 35.0]), Some([116.0, 35.0])],
            inventory_x: 8.0,
            inventory_y: 84.0,
            own_labels: false,
            own_background: false,
        })
    }

    fn screen_draw(&self, _: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
        crate::tour::reload::draw(event);
        crate::tour::client::overlay(event);
        let [x, y] = event.origin;
        event
            .draw
            .fill([x + 66.0, y + 55.0, 44.0, 16.0], 0xff555555);
        event
            .draw
            .text("Deposit", [x + 68.0, y + 59.0], 0xffffffff, false);
        let duration = event.data.get(1).copied().unwrap_or(DURATION).max(1);
        let progress = event
            .data
            .first()
            .copied()
            .unwrap_or_default()
            .clamp(0, duration);
        event.draw.fill([x + 72.0, y + 40.0, 32.0, 6.0], 0xff555555);
        event.draw.fill(
            [
                x + 72.0,
                y + 40.0,
                32.0 * progress as f32 / duration as f32,
                6.0,
            ],
            0xff55bb55,
        );
    }

    fn screen_click(&self, _: &dyn ClientHost, event: &mut ScreenClickEvent<'_>) -> Option<i32> {
        (!event.right
            && (66.0..110.0).contains(&event.at[0])
            && (55.0..71.0).contains(&event.at[1]))
        .then_some(DEPOSIT)
    }

    fn menu_button(&self, host: &mut dyn ServerHost, event: &mut MenuButtonEvent<'_>) {
        if event.context.id != DEPOSIT {
            return;
        }
        let (Some(input), Some(stack)) = (event.entity.items.first_mut(), event.carried.as_ref())
        else {
            return;
        };
        let limit = host.max_stack_size(stack.item()).unwrap_or(0);
        let compatible = input
            .as_ref()
            .is_none_or(|item| item.is_stackable_with(stack));
        let count = deposit_count(
            stack.count(),
            input.as_ref().map_or(0, ItemStack::count),
            limit,
            compatible,
        );
        if count == 0 {
            return;
        }
        if let Some(input) = input {
            input
                .set_count(input.count() + count)
                .expect("deposit respects capacity");
            *event.carried = event.carried.take().and_then(|stack| stack.shrink(count));
        } else {
            let split = event.carried.take().unwrap().split(count);
            *input = split.taken;
            *event.carried = split.remainder;
        }
    }
}
