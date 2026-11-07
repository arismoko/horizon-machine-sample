//! A remappable inspection view and a small machine-screen overlay.
use mod_api::*;

use crate::MACHINE;

pub fn register(registrar: &mut dyn Registrar) {
    registrar.register_key_mapping(
        NativeKeyRegistration::new("inspect", "key.keyboard.i")
            .label("key.machine_sample.inspect")
            .context(KeyConflictContext::InGame),
    );
}

pub fn input(host: &dyn ClientHost, event: &InputKeyEvent<'_>) {
    if event.action != KeyAction::Press || !host.playing() {
        return;
    }
    let Some((key, modifiers)) = host.mod_key("machine_sample", "inspect") else {
        return;
    };
    if event.key != key || event.modifiers & 7 != modifiers.bits & 7 {
        return;
    }
    let Some(NativeCrosshairTarget::Block { state, .. }) = host.crosshair_target() else {
        return;
    };
    let Some(state) = host.block_state_string(state) else {
        return;
    };
    if state.split('[').next() != Some(MACHINE) {
        return;
    }
    host.set_camera_type(inspection_view(host.camera_type()));
    let _ = host.play_local_sound(&SoundPlay::ui("minecraft:ui.button.click"), 0);
}

fn inspection_view(current: CameraType) -> CameraType {
    if current.is_first_person() {
        CameraType::ThirdPersonBack
    } else {
        CameraType::FirstPerson
    }
}

pub fn overlay(event: &mut ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    event
        .draw
        .text("Processing cycle", [x + 8.0, y + 18.0], 0xffaaaaaa, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspection_view_returns_from_either_third_person_mode() {
        assert_eq!(
            inspection_view(CameraType::FirstPerson),
            CameraType::ThirdPersonBack
        );
        assert_eq!(
            inspection_view(CameraType::ThirdPersonBack),
            CameraType::FirstPerson
        );
        assert_eq!(
            inspection_view(CameraType::ThirdPersonFront),
            CameraType::FirstPerson
        );
    }
}
