//! A saved machine tank, filled or drained with the player's held bucket.
use mod_api::*;

pub fn tanks() -> ModFluidTanks {
    let mut tanks = ModFluidTanks::default();
    tanks.tanks = vec![4_000];
    tanks.faces = std::array::from_fn(|_| vec![0]);
    tanks.inputs = vec![0];
    tanks.outputs = vec![0];
    tanks
}

pub fn register(event: &mut RegisterCommandsEvent<'_>) {
    for verb in ["fill", "drain"] {
        event.commands.push(Command::op(
            &format!("machine_sample {verb} <tank:block_pos>"),
            2,
        ));
    }
}

fn direction(command: &CommandInvocation) -> Option<bool> {
    match command.literals.as_slice() {
        [root, verb] if root == "machine_sample" && verb == "fill" => Some(true),
        [root, verb] if root == "machine_sample" && verb == "drain" => Some(false),
        _ => None,
    }
}

pub fn run(
    host: &mut dyn ServerHost,
    event: &mut RunModCommandEvent<'_>,
    machine: Option<BlockKey>,
) -> Flow {
    let Some(fill) = direction(event.command) else {
        return Flow::Continue;
    };
    let CommandSource::Player { player, .. } = event.source else {
        let _ = host
            .commands()
            .feedback(Text::literal("Hold a bucket and run this as a player."));
        return Flow::Cancel;
    };
    let Some(CommandArgumentValue::BlockPos(pos)) = event.command.argument("tank") else {
        return Flow::Continue;
    };
    let Some(level) = player.entity().level(host) else {
        let _ = host
            .commands()
            .feedback(Text::literal("The player is no longer in a level."));
        return Flow::Cancel;
    };
    if !level
        .block(host, *pos)
        .is_some_and(|block| Some(block) == machine)
    {
        let _ = host
            .commands()
            .feedback(Text::literal("Target a loaded sample machine."));
        return Flow::Cancel;
    }
    let tank = level.capability(host, &FLUIDS, *pos, None);
    let held = ItemPlace::PlayerHand {
        player: player.uuid(),
        hand: Hand::Main,
    };
    let bucket = held.capability(host, &ITEM_FLUIDS);
    let (Some(tank), Some(bucket)) = (tank, bucket) else {
        let _ = host.commands().feedback(Text::literal(
            "No fluid handler in the machine or held item.",
        ));
        return Flow::Cancel;
    };
    let (from, to) = if fill {
        (&bucket, &tank)
    } else {
        (&tank, &bucket)
    };
    let answer = host.transaction(|host, tx| move_resources(host, from, to, |_| true, 1_000, tx));
    let feedback = match answer {
        Ok(amount) => format!("Transferred {amount} mB."),
        Err(_) => "Transfer refused; no fluid moved.".into(),
    };
    let _ = host.commands().feedback(Text::literal(feedback));
    Flow::Cancel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tank_has_four_bucket_capacity_and_all_faces() {
        let tank = tanks();
        assert_eq!(tank.tanks, [4_000]);
        assert!(tank.faces.iter().all(|face| face == &[0]));
        assert_eq!(tank.inputs, [0]);
        assert_eq!(tank.outputs, [0]);
    }

    #[test]
    fn routes_only_our_transfer_commands() {
        let command = |verb: &str| {
            CommandInvocation::new(
                vec!["machine_sample".into(), verb.into()],
                vec![],
                mod_api::DimensionKey::OVERWORLD,
                [0.0; 3],
            )
        };
        assert_eq!(direction(&command("fill")), Some(true));
        assert_eq!(direction(&command("drain")), Some(false));
        assert_eq!(direction(&command("announce")), None);
    }
}
