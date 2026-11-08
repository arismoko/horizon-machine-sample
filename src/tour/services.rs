//! An operator announcement command using parsed input and server services.
use mod_api::*;

pub fn register(event: &mut RegisterCommandsEvent<'_>) {
    event.commands.push(NativeCommand::op(
        "machine_sample announce <message:text>",
        2,
    ));
}

pub fn run(host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
    let Some(message) = announcement(event.command, event.source) else {
        return Flow::Continue;
    };
    let feedback = if host.messages().broadcast(Text::literal(message)).is_ok() {
        "Announcement delivered."
    } else {
        "Announcement could not be delivered."
    };
    // The command service, rather than an acting-player lookup, routes feedback
    // correctly for console, functions, command blocks and players alike.
    let _ = host.commands().feedback(Text::literal(feedback));
    Flow::Cancel
}

fn announcement(command: &CommandInvocation, source: &CommandSource) -> Option<String> {
    if command.literals != ["machine_sample", "announce"] {
        return None;
    }
    let CommandArgumentValue::String(message) = command.argument("message")? else {
        return None;
    };
    let origin = match source {
        CommandSource::Player { .. } => "player",
        CommandSource::Console { .. } => "console",
        CommandSource::CommandBlock { .. } => "command block",
        CommandSource::CommandBlockMinecart { .. } => "command minecart",
        CommandSource::Function { .. } => "function",
        _ => "server",
    };
    Some(format!("[Machine Sample / {origin}] {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(literals: &[&str], message: &str) -> CommandInvocation {
        CommandInvocation::new(
            literals.iter().map(|s| (*s).to_owned()).collect(),
            vec![CommandArgument::new(
                "message".into(),
                CommandArgumentValue::String(message.into()),
            )],
        )
    }

    #[test]
    fn announces_parsed_text_and_typed_origin() {
        let source = CommandSource::Console {
            permission: NativePermissionLevel::All,
        };
        assert_eq!(
            announcement(
                &command(&["machine_sample", "announce"], "Machine ready"),
                &source
            ),
            Some("[Machine Sample / console] Machine ready".into())
        );
        assert_eq!(
            announcement(&command(&["another_mod", "announce"], "ignored"), &source),
            None
        );
        let function = CommandSource::Function {
            id: Identifier::parse("machine_sample:alarm").unwrap(),
            permission: NativePermissionLevel::All,
        };
        assert_eq!(
            announcement(
                &command(&["machine_sample", "announce"], "Ready"),
                &function
            ),
            Some("[Machine Sample / function] Ready".into())
        );
    }
}
