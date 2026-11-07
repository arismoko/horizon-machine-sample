//! An API-only function-slot listener, not a raw mixin or a second hook route.
use mod_api::{HostSide, Text};
use mod_points::OnTickRateChangedArgs;

pub fn tick_rate_seen(host: HostSide<'_>, _args: &mut OnTickRateChangedArgs) {
    if let HostSide::Server(host) = host {
        host.messages()
            .broadcast(Text::literal("Machine Sample: tick-rate update observed."));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn listener_has_the_pinned_api_head_signature() {
        // Signature compatibility and slot owner id only, not dispatch or listener behavior.
        let slot = mod_points::listeners::on_tick_rate_changed(
            "machine_sample",
            0,
            "tick_rate_seen",
            crate::tick_rate_seen,
        );
        assert_eq!(slot.id(), "machine_sample");
    }
}
