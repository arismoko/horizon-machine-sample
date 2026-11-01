fn main() {
    let mods: Vec<Box<dyn horizon_client::mod_api::Hooks>> =
        vec![Box::new(machine_hooks::MachineHooks::from_env())];
    horizon_client::entry::main_with(mods.into());
}
