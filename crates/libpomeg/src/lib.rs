use crate::checksum::is_valid_checksum;
use byteorder::{ByteOrder, LittleEndian};

mod checksum;

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

#[derive(Debug)]
pub struct Gen3Save {
    save_slot: SaveSlot,
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
