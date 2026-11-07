//! Read sample-owned data and assets once per side-typed pack reload.
use mod_api::*;
use std::{
    collections::{BTreeMap, HashMap},
    io,
    sync::{Arc, LazyLock, OnceLock, RwLock},
};

static TIMES: LazyLock<RwLock<Durations>> = LazyLock::new(RwLock::default);
static LABEL: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("Machine processing".into()));

pub fn server(event: &mut AddServerReloadListenersEvent<'_>) {
    event
        .listeners
        .add_listener(
            Identifier::parse("machine_sample:processing_times").unwrap(),
            FileToIdConverter::json("machine_tour").unwrap(),
            Arc::new(Times),
        )
        .expect("unique sample server listener");
}

pub fn client(event: &mut AddClientReloadListenersEvent<'_>) {
    event
        .listeners
        .add_listener(
            Identifier::parse("machine_sample:screen_label").unwrap(),
            FileToIdConverter::json("machine_tour").unwrap(),
            Arc::new(Label),
        )
        .expect("unique sample client listener");
}

#[derive(Default)]
struct Durations {
    entries: Vec<(Identifier, i32)>,
    resolved: OnceLock<HashMap<ItemKey, i32>>,
}

impl Durations {
    fn duration(&self, host: &dyn Host, item: ItemKey) -> i32 {
        // Prepare has no resolver. Resolve once on the first server lookup, never per tick.
        self.resolved.get_or_init(|| self.entries.iter().filter_map(|(id, ticks)| {
            match host.item_key(id) {
                Some(key) => Some((key, *ticks)),
                None => {
                    log::warn!(target: "machine_sample::reload", "Skipping processing time for unknown item {id}");
                    None
                }
            }
        }).collect()).get(&item).copied().unwrap_or(crate::DURATION)
    }
}

struct Times;
impl ServerReloadListener for Times {
    fn prepare(
        &self,
        context: &ServerReloadContext,
    ) -> Result<Box<dyn ReloadApply>, ReloadResourceError> {
        let id = Identifier::parse("machine_sample:processing").unwrap();
        let bytes = context.read(&id).ok_or_else(|| {
            ReloadResourceError::new(
                id.clone(),
                io::Error::new(io::ErrorKind::NotFound, "missing processing table"),
            )
        })?;
        let table = parse_times(bytes).map_err(|error| ReloadResourceError::new(id, error))?;
        Ok(<dyn ReloadApply>::new(move || {
            *TIMES.write().unwrap_or_else(|e| e.into_inner()) = table;
        }))
    }
}

fn parse_times(bytes: &[u8]) -> io::Result<Durations> {
    let table: BTreeMap<String, i32> = serde_json::from_slice(bytes)?;
    let entries = table
        .into_iter()
        .map(|(item, ticks)| {
            let id = Identifier::parse_namespaced(&item).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "expected a namespaced item id")
            })?;
            if !(1..=10_000).contains(&ticks) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "processing ticks must be in 1..=10000",
                ));
            }
            Ok((id, ticks))
        })
        .collect::<io::Result<_>>()?;
    Ok(Durations {
        entries,
        resolved: OnceLock::new(),
    })
}

struct Label;
impl ClientReloadListener for Label {
    fn prepare(
        &self,
        context: &ClientReloadContext,
    ) -> Result<Box<dyn ReloadApply>, ReloadResourceError> {
        let id = Identifier::parse("machine_sample:screen").unwrap();
        let bytes = context.read(&id).ok_or_else(|| {
            ReloadResourceError::new(
                id.clone(),
                io::Error::new(io::ErrorKind::NotFound, "missing screen label"),
            )
        })?;
        let label = parse_label(bytes).map_err(|error| ReloadResourceError::new(id, error))?;
        Ok(<dyn ReloadApply>::new(move || {
            *LABEL.write().unwrap_or_else(|e| e.into_inner()) = label;
        }))
    }
}

fn parse_label(bytes: &[u8]) -> io::Result<String> {
    let label: String = serde_json::from_slice(bytes)?;
    if label.chars().count() > 24 || label.contains(['\r', '\n']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "screen label must be one line, at most 24 characters",
        ));
    }
    Ok(label)
}

pub fn duration(host: &dyn Host, item: ItemKey) -> i32 {
    TIMES
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .duration(host, item)
}

pub fn draw(event: &mut ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let label = LABEL.read().unwrap_or_else(|e| e.into_inner());
    event
        .draw
        .text(&label, [x + 8.0, y - 12.0], 0xffaaaaaa, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::prelude::ItemsHost;

    #[test]
    fn validates_processing_table_once_at_reload() {
        let table = parse_times(br#"{"minecraft:cobblestone":80}"#).unwrap();
        assert_eq!(
            table.entries,
            vec![(Identifier::parse("minecraft:cobblestone").unwrap(), 80)]
        );
        assert!(table.resolved.get().is_none());
        for bad in [
            br#"{"minecraft:cobblestone":0}"#.as_slice(),
            br#"{"minecraft:cobblestone":10001}"#,
            br#"{"invalid id":40}"#,
            br#"{"minecraft:cobblestone":"40"}"#,
        ] {
            assert!(parse_times(bad).is_err());
        }
    }

    #[test]
    fn replacing_reload_data_drops_keys_and_warns_once_per_missing_item() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct Warnings(AtomicUsize);
        impl log::Log for Warnings {
            fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
                metadata.target() == "machine_sample::reload"
            }
            fn log(&self, record: &log::Record<'_>) {
                if self.enabled(record.metadata()) {
                    self.0.fetch_add(1, Ordering::Relaxed);
                }
            }
            fn flush(&self) {}
        }
        static WARNINGS: Warnings = Warnings(AtomicUsize::new(0));
        log::set_logger(&WARNINGS).unwrap();
        log::set_max_level(log::LevelFilter::Warn);
        let mut host = mod_api::testing::FakeHost::new();
        let id = Identifier::parse("minecraft:cobblestone").unwrap();
        host.registries.resolve_items([id.clone()]).unwrap();
        let key = host.item_key(&id).unwrap();
        let mut table =
            parse_times(br#"{"minecraft:cobblestone":80,"machine_sample:missing":20}"#).unwrap();
        assert_eq!(table.duration(&host, key), 80);
        assert_eq!(table.duration(&host, key), 80);
        assert_eq!(WARNINGS.0.load(Ordering::Relaxed), 1);
        table =
            parse_times(br#"{"minecraft:cobblestone":20,"machine_sample:missing":10}"#).unwrap();
        assert!(table.resolved.get().is_none());
        assert_eq!(table.duration(&host, key), 20);
        assert_eq!(WARNINGS.0.load(Ordering::Relaxed), 2);
        table = parse_times(br#"{}"#).unwrap();
        assert_eq!(table.duration(&host, key), crate::DURATION);
        assert_eq!(WARNINGS.0.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn validates_client_label() {
        assert_eq!(parse_label(br#""Working""#).unwrap(), "Working");
        assert!(parse_label(br#""two\nlines""#).is_err());
        assert!(parse_label(br#""this label is far too long for this screen""#).is_err());
    }
}
