//! Prebuilt API-only validation using the pinned game's shared constructor resolver.
use horizon_mod_package::{Manifest, manifest::inject::Access};
use std::{env, fs, path::PathBuf};

fn run() -> Result<(), String> {
    let args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    let [source, manifest] = args.as_slice() else {
        return Err("usage: check-injections SOURCE MANIFEST".into());
    };
    let manifest = Manifest::parse(&fs::read_to_string(manifest).map_err(|e| e.to_string())?)?;
    for entry in &manifest.inject {
        if entry.access != Access::Api || entry.call.is_some() {
            return Err(format!(
                "{}: this sample check supports only prebuilt API function slots",
                entry.id
            ));
        }
    }
    if manifest.inject.is_empty() {
        return Ok(());
    }
    let catalog = horizon_mod_api_gen::points::game_points(source)?;
    let points: Vec<horizon_mod_point_emit::Point> =
        serde_json::from_value(serde_json::to_value(catalog).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    horizon_mod_point_emit::api(&manifest, &points, "")?;
    println!("validated {} API injections", manifest.inject.len());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Injection validation: {error}");
        std::process::exit(1);
    }
}
