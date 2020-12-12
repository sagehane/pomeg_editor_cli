use byteorder::{ByteOrder, LittleEndian};

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

#[derive(Debug)]
pub struct Gen3Save {
    save_slot: SaveSlot,
}

#[derive(Debug)]
enum Gen3Game {
    // Ruby & Sapphire
    RS = 0,
    // Fire Red & Leaf Green
    FRLG = 1,
    // Emerald
    E,
}

#[derive(Debug)]
enum SaveSlot {
    A = 0,
    B = 1,
}

impl Gen3Save {
    pub fn from_buffer(buffer: &[u8; 0x20000]) -> Self {
        let save_slot = match slot_from_buffer(buffer) {
            Some(s) => s,
            None => panic!("No valid save slot"),
        };

        if !is_valid_checksum(buffer) {
            panic!("Checksum is invalid");
        }

        Gen3Save { save_slot }
    }
}

/// Find out the slot from given buffer
fn slot_from_buffer(buffer: &[u8; 0x20000]) -> Option<SaveSlot> {
    let mut save_index: u32 = get_save_index(sector_by_id(0, buffer));
    let mut save_slot = None;

    for sector_id in 0..27 {
        let retrieved_index = get_save_index(sector_by_id(sector_id, buffer));

        if sector_id == 14 {
            if retrieved_index < save_index {
                save_slot = Some(SaveSlot::A);
            } else if retrieved_index > save_index {
                save_slot = Some(SaveSlot::B);
            } else {
                eprintln!("Slot A and B has the same save_index");

                return None;
            }

            save_index = retrieved_index;
        } else if sector_id != 0 && save_index != retrieved_index {
            eprintln!(
                "Sector {} has an invalid save_index, expected \"{}\" but got \"{}\"",
                sector_id, save_index, retrieved_index
            );

            return None;
        }
    }

    save_slot
}

/// Gets the save index from a sector
fn get_save_index(sector: &[u8]) -> u32 {
    LittleEndian::read_u32(&sector[0x0FFC..0x1000])
}

/// Take an buffer and id to return a sector in the form of a slice
///
/// # Panics
///
/// Panics when `sector_id > 31`
fn sector_by_id(sector_id: u8, buffer: &[u8]) -> &[u8] {
    if sector_id > 31 {
        panic!("Sector must be between 0 and 31")
    }

    let sector_offset: usize = (sector_id as usize) << 12;

    &buffer[sector_offset..sector_offset + 0x1000]
}

/// Checks if the save file has the correct checksum. Currently only checks through sectors 0 to
/// 27, leaving 28 to 31 unchecked.
fn is_valid_checksum(buffer: &[u8; 0x20000]) -> bool {
    for sector_id in 0..27 {
        if !is_valid_sector(sector_by_id(sector_id, buffer)) {
            eprintln!("Sector {} has an invalid checksum", sector_id);

            return false;
        }
    }

    true
}

/// Checks if the checksum of a sector is valid by comparing the value of the two bytes stored in
/// offset of 0xff4 with the value from `calculate_checksum`.
fn is_valid_sector(sector: &[u8]) -> bool {
    let calculated_checksum =
        calculate_checksum(&sector[..SECTOR_SIZE[*&sector[0xFF4] as usize] as usize]);

    let checksum = LittleEndian::read_u16(&sector[0xFF6..0xFF8]);

    if calculated_checksum != checksum {
        return false;
    }

    true
}

/// Separates a slice into litte-endian u32 values, gets the total sum of the slice, and gets the
/// sum of the upper and lower 16 bytes.
fn calculate_checksum(sector: &[u8]) -> u16 {
    let mut checksum: u32 = 0;

    for i in (0..sector.len()).step_by(4) {
        checksum = checksum.wrapping_add(LittleEndian::read_u32(&sector[i..i + 4]));
    }

    ((checksum >> 16) as u16).wrapping_add(checksum as u16)
}
