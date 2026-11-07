//! Read sample-owned data and assets once per side-typed pack reload.
use mod_api::*;
use std::{
    collections::{BTreeMap, HashMap},
    io,
    sync::{Arc, LazyLock, RwLock},
};

static TIMES: LazyLock<RwLock<HashMap<ItemKey, i32>>> = LazyLock::new(RwLock::default);
static LABEL: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("Machine processing".into()));

pub fn server(
    event: &mut AddServerReloadListenersEvent<'_>,
    items: Arc<BTreeMap<String, ItemKey>>,
) {
    event
        .listeners
        .add_listener(
            Identifier::parse("machine_sample:processing_times").unwrap(),
            FileToIdConverter::json("machine_tour").unwrap(),
            Arc::new(Times { items }),
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

struct Times {
    items: Arc<BTreeMap<String, ItemKey>>,
}
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
        let table =
            parse_times(bytes, &self.items).map_err(|error| ReloadResourceError::new(id, error))?;
        Ok(<dyn ReloadApply>::new(move || {
            *TIMES.write().unwrap_or_else(|e| e.into_inner()) = table;
        }))
    }
}

fn parse_times(
    bytes: &[u8],
    items: &BTreeMap<String, ItemKey>,
) -> io::Result<HashMap<ItemKey, i32>> {
    let table: BTreeMap<String, i32> = serde_json::from_slice(bytes)?;
    table
        .into_iter()
        .map(|(item, ticks)| {
            let key = items.get(&item).copied().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "expected a registered namespaced item id",
                )
            })?;
            if !(1..=10_000).contains(&ticks) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "processing ticks must be in 1..=10000",
                ));
            }
            Ok((key, ticks))
        })
        .collect()
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

pub fn duration(item: ItemKey) -> i32 {
    TIMES
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .get(&item)
        .copied()
        .unwrap_or(crate::DURATION)
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

    #[test]
    fn validates_processing_table_once_at_reload() {
        let mut host = mod_api::testing::FakeHost::new();
        let id = Identifier::parse("minecraft:cobblestone").unwrap();
        host.registries.resolve_items([id.clone()]).unwrap();
        let key = host.item_key(&id).unwrap();
        let items = BTreeMap::from([(id.to_string(), key)]);
        assert_eq!(
            parse_times(br#"{"minecraft:cobblestone":80}"#, &items).unwrap()[&key],
            80
        );
        assert_eq!(
            parse_times(br#"{"minecraft:cobblestone":20}"#, &items).unwrap()[&key],
            20
        );
        assert!(parse_times(br#"{}"#, &items).unwrap().is_empty());
        for bad in [
            br#"{"minecraft:cobblestone":0}"#.as_slice(),
            br#"{"minecraft:cobblestone":10001}"#,
            br#"{"invalid id":40}"#,
            br#"{"minecraft:unregistered":40}"#,
            br#"{"minecraft:cobblestone":"40"}"#,
        ] {
            assert!(parse_times(bad, &items).is_err());
        }
    }

    #[test]
    fn validates_client_label() {
        assert_eq!(parse_label(br#""Working""#).unwrap(), "Working");
        assert!(parse_label(br#""two\nlines""#).is_err());
        assert!(parse_label(br#""this label is far too long for this screen""#).is_err());
    }
}
