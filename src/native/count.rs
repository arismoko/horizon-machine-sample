/// Observe mod block-entity ticks without changing their Part behaviour.
#[derive(Default)]
pub struct Machines {
    ticked: u64,
}

impl BlockEntityTickSlot for Machines {
    fn after(&mut self, tick: &mut BlockEntityTick<'_, '_>) {
        if tick.kind == TickedKind::Mod {
            self.ticked += 1;
            if self.ticked == 1000 {
                eprintln!("machine_sample: loop observed 1000 mod block-entity ticks");
            }
        }
    }
}
