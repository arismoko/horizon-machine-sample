// Shared by the standalone tests and the native bundle.
mod machine;
mod processing;
mod tour;

use mod_api::*;

#[derive(Default)]
struct Sample {
    recipes: std::sync::Arc<processing::ProcessingRecipes>,
    machine: std::sync::OnceLock<BlockKey>,
}

#[mod_api::horizon_mod]
impl Hooks for Sample {
    fn server_tick_rate_changed(
        &self,
        host: &mut dyn ServerHost,
        _: &mut ServerTickRateChangedEvent,
    ) {
        let _ = host
            .messages()
            .broadcast(Text::literal("Machine Sample: tick-rate update observed."));
    }

    fn server_about_to_start(&self, _: &mut dyn ServerHost, _: &mut ServerAboutToStartEvent) {
        self.recipes.clear();
    }

    fn datapack_sync(&self, host: &mut dyn ServerHost, _: &mut DatapackSyncEvent<'_>) {
        self.recipes.publish(host);
    }

    fn server_stopped(&self, _: &mut dyn ServerHost, _: &mut ServerStoppedEvent) {
        self.recipes.clear();
    }

    fn entity_interact(
        &self,
        host: HostSide<'_>,
        event: &mut EntityInteractEvent<'_>,
    ) -> Option<InteractionResult> {
        // The visit count is the server's: the client predicts nothing here.
        match host {
            HostSide::Server(host) => tour::attachments::inspect(host, event),
            HostSide::Client(_) => None,
        }
    }

    fn add_server_reload_listeners(
        &self,
        _: &dyn Host,
        event: &mut AddServerReloadListenersEvent<'_>,
    ) {
        tour::reload::server(event);
    }

    fn add_client_reload_listeners(
        &self,
        _: &dyn ClientHost,
        event: &mut AddClientReloadListenersEvent<'_>,
    ) {
        tour::reload::client(event);
    }

    fn input_key(&self, host: &mut dyn ClientHost, event: &mut InputKeyEvent<'_>) {
        tour::client::input(host, event);
    }

    fn register_commands(&self, _: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        tour::services::register(event);
        tour::fluids::register(event);
    }

    fn run_mod_command(
        &self,
        host: &mut dyn ServerHost,
        event: &mut RunModCommandEvent<'_>,
    ) -> Flow {
        if matches!(tour::services::run(host, event), Flow::Cancel) {
            return Flow::Cancel;
        }
        tour::fluids::run(host, event, self.machine.get().copied())
    }

    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        let machine_block = registrar.block_key(MACHINE);
        registrar.block_container(&machine_block, container());
        registrar.block_fluids(&machine_block, tour::fluids::tanks());
        registrar.block_energy(&machine_block, tour::energy::buffer());
        let _ = self.machine.set(machine_block);
        let pipe = registrar.block_key(PIPE);
        tour::client::register(registrar);
        let automatic_input = tour::policy::register(registrar);
        let machine = registrar
            .block_entity_type("machine")
            .blocks(&[MACHINE])
            .build();
        registrar
            .block_entity_part(
                &machine,
                machine::Machine {
                    pipe,
                    automatic_input,
                    recipes: self.recipes.clone(),
                },
            )
            .expect("native block entity type");
    }
}

fn container() -> ModContainer {
    ModContainer {
        slots: 5,
        faces: std::array::from_fn(|_| vec![0, 1]),
        inputs: vec![0],
        outputs: vec![1],
        menu: Some("minecraft:hopper".into()),
    }
}

pub const MACHINE: &str = "machine_sample:machine";
pub const PIPE: &str = "machine_sample:pipe";
pub const DURATION: i32 = 40;
pub const DEPOSIT: i32 = 0;

pub fn fill_empty<T>(slot: &mut Option<T>, extract: impl FnOnce() -> Option<T>) {
    if slot.is_none() {
        *slot = extract();
    }
}

pub fn deposit_count(carried: u32, input: u32, limit: u32, compatible: bool) -> u32 {
    if compatible {
        carried.min(limit.saturating_sub(input))
    } else {
        0
    }
}

pub fn advance(progress: i32, can_process: bool, duration: i32) -> (i32, bool) {
    if !can_process {
        return (0, false);
    }
    let next = progress.clamp(0, duration - 1) + 1;
    if next == duration {
        (0, true)
    } else {
        (next, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fills_only_an_empty_slot() {
        let mut slot = None;
        fill_empty(&mut slot, || None::<u8>);
        assert_eq!(slot, None);
        fill_empty(&mut slot, || Some(7));
        assert_eq!(slot, Some(7));
        fill_empty(&mut slot, || {
            panic!("must not extract into an occupied slot")
        });
        assert_eq!(slot, Some(7));
    }
    #[test]
    fn deposit_respects_capacity_and_compatibility() {
        assert_eq!(deposit_count(0, 0, 64, true), 0);
        assert_eq!(deposit_count(32, 0, 64, true), 32);
        assert_eq!(deposit_count(32, 50, 64, true), 14);
        assert_eq!(deposit_count(32, 64, 64, true), 0);
        assert_eq!(deposit_count(32, 70, 64, true), 0);
        assert_eq!(deposit_count(32, 5, 64, false), 0);
    }
    #[test]
    fn completes_after_forty_ticks_and_resets_when_blocked() {
        let mut progress = 0;
        for _ in 0..39 {
            let (next, done) = advance(progress, true, DURATION);
            assert!(!done);
            progress = next;
        }
        assert_eq!(advance(progress, true, DURATION), (0, true));
        assert_eq!(advance(progress, false, DURATION), (0, false));
        assert_eq!(advance(79, true, 80), (0, true));
        assert_eq!(advance(0, true, 1), (0, true));
    }
}
