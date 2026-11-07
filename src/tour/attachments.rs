//! Crouch with an empty hand and inspect an entity to record a saved visit.
use mod_api::*;

const VISITS: AttachmentType<u32> = AttachmentType {
    namespace: "machine_sample",
    name: "inspection_visits",
    default: || 0,
    codec: Some(AttachmentCodec { encode, decode }),
    copy_on_death: false,
    sync: false,
};

fn encode(visits: &u32) -> Result<Compound, AttachmentError> {
    let mut saved = Compound::default();
    saved.insert("visits".into(), Tag::Long(i64::from(*visits)));
    Ok(saved)
}

fn decode(saved: &Compound) -> Result<u32, AttachmentError> {
    let Some(Tag::Long(visits)) = saved.get(&"visits".into()) else {
        return Err(AttachmentError::Decode);
    };
    u32::try_from(*visits).map_err(|_| AttachmentError::Decode)
}

pub fn inspect(host: &mut dyn ServerHost, event: &EntityInteractEvent<'_>) -> Flow {
    let Some(player) = host.players().acting_player() else {
        return Flow::Continue;
    };
    if player.is_secondary_use_active(host) != Some(true) {
        return Flow::Continue;
    }
    let main = ItemPlace::PlayerHand {
        player: player.uuid(),
        hand: Hand::Main,
    }
    .stack(host);
    if !inspection_hands_empty(main.as_ref(), event.stack) {
        return Flow::Continue;
    }
    let Ok(id) = u32::try_from(event.entity) else {
        return Flow::Continue;
    };
    // The event's target belongs to the host's current level. No entity, world
    // borrow or holder identity is retained beyond this callback.
    let holder = Holder::Entity(id);
    let answer = VISITS.get_existing(host, holder).and_then(|visits| {
        let visits = visits.unwrap_or_default().saturating_add(1);
        VISITS.set(host, holder, visits)?;
        Ok(visits)
    });
    match answer {
        Ok(visits) => {
            let _ = player.action_bar(
                host,
                Text::literal(format!("Inspected {}: visit {visits}", event.kind)),
            );
            // Claim this gesture so vanilla does not retry with the other hand.
            Flow::Cancel
        }
        Err(AttachmentError::MissingHolder) => Flow::Continue,
        Err(_) => {
            let _ = player.action_bar(
                host,
                Text::literal("Inspection data could not be read; left unchanged."),
            );
            Flow::Continue
        }
    }
}

fn inspection_hands_empty(main: Option<&ItemStack>, used: Option<&ItemStack>) -> bool {
    main.is_none() && used.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::prelude::ItemsHost;

    #[test]
    fn an_empty_off_hand_does_not_bypass_an_occupied_main_hand() {
        let mut host = mod_api::testing::FakeHost::new();
        let id = Identifier::parse("minecraft:stick").unwrap();
        host.registries.resolve_items([id.clone()]).unwrap();
        let stick = ItemStack::of(host.item_key(&id).unwrap());
        assert!(inspection_hands_empty(None, None));
        assert!(!inspection_hands_empty(Some(&stick), None));
        assert!(!inspection_hands_empty(None, Some(&stick)));
        assert!(!inspection_hands_empty(Some(&stick), Some(&stick)));
    }

    #[test]
    fn persisted_visit_count_round_trips_without_truncation() {
        for visits in [0, 1, u32::MAX] {
            assert_eq!(decode(&encode(&visits).unwrap()), Ok(visits));
        }
    }

    #[test]
    fn malformed_saved_count_is_refused() {
        assert_eq!(decode(&Compound::default()), Err(AttachmentError::Decode));
        for value in [
            Tag::Int(1),
            Tag::Long(-1),
            Tag::Long(i64::from(u32::MAX) + 1),
        ] {
            let mut saved = Compound::default();
            saved.insert("visits".into(), value);
            assert_eq!(decode(&saved), Err(AttachmentError::Decode));
        }
    }
}
