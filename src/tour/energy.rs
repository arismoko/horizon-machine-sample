//! The machine recovers energy on a completed recipe and exposes a saved buffer.
use mod_api::*;

pub const CAPACITY: i32 = 1_000;
const PER_RECIPE: i32 = 25;

pub fn buffer() -> ModEnergyBuffer {
    ModEnergyBuffer {
        capacity: CAPACITY,
        max_insert: 100,
        max_extract: 100,
        insert_faces: [true; 6],
        extract_faces: [true; 6],
    }
}

pub fn charge(nbt: &mut Compound) {
    let amount = match nbt.get(&"energy".into()) {
        Some(Tag::Int(amount)) => *amount,
        _ => 0,
    };
    // The canonical ENERGY provider uses this same saved NBT field. Updating
    // the ticking Part's entity avoids looking up its temporarily lifted state.
    nbt.insert(
        "energy".into(),
        Tag::Int(
            amount
                .clamp(0, CAPACITY)
                .saturating_add(PER_RECIPE)
                .min(CAPACITY),
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_energy_is_saved_and_bounded() {
        let mut nbt = Compound::default();
        charge(&mut nbt);
        assert_eq!(nbt.get(&"energy".into()), Some(&Tag::Int(25)));
        nbt.insert("energy".into(), Tag::Int(990));
        charge(&mut nbt);
        assert_eq!(nbt.get(&"energy".into()), Some(&Tag::Int(CAPACITY)));
        charge(&mut nbt);
        assert_eq!(nbt.get(&"energy".into()), Some(&Tag::Int(CAPACITY)));
        assert_eq!(buffer().capacity, CAPACITY);
        assert_eq!(buffer().max_extract, 100);
    }
}
