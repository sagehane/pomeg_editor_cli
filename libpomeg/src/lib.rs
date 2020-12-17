use crate::checksum::is_valid_checksum;
pub use crate::save::{DataStructure, Save, Sector};
use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryInto;

mod checksum;
mod save;

#[derive(Debug)]
pub struct SaveStruct {
    save_slot: SaveSlot,
    trainer_id: TrainerID,
    trainer_name: [u8; 7],
}

impl SaveStruct {
    pub fn from_save(save: Save) -> Self {
        let save_slot = match slot_from_save(save) {
            Some(s) => s,
            None => panic!("No valid save slot"),
        };

        if !is_valid_checksum(save) {
            panic!("Checksum is invalid");
        }

        let trainer_id = TrainerID::from_sector(save[save_slot.sector_offset(1) as usize]);

        let trainer_name = save[save_slot.sector_offset(1) as usize][0..7]
            .try_into()
            .unwrap();

        SaveStruct {
            save_slot,
            trainer_id,
            trainer_name,
        }
    }
}

#[derive(Debug)]
enum SaveSlot {
    A = 0,
    B = 1,
}

impl SaveSlot {
    fn sector_offset(&self, sector_id: u8) -> u8 {
        match self {
            SaveSlot::A => sector_id,
            SaveSlot::B => sector_id + 14,
        }
    }
}

#[derive(Debug)]
struct TrainerID {
    public: u16,
    secret: u16,
}

impl TrainerID {
    fn from_sector(sector: Sector) -> Self {
        let public = LittleEndian::read_u16(&sector[0xA..0xC]);
        let secret = LittleEndian::read_u16(&sector[0xD..0xF]);

        TrainerID { public, secret }
    }
}

/// Find out the slot from given save
fn slot_from_save(save: Save) -> Option<SaveSlot> {
    let mut save_index: u32 = get_save_index(save[0]);
    let mut save_slot = None;

    for sector_id in 0..28 {
        let retrieved_index = get_save_index(save[sector_id]);

        if sector_id == 14 {
            if save_index != u32::MAX && retrieved_index < save_index {
                save_slot = Some(SaveSlot::A);
            } else if save_index == u32::MAX || retrieved_index > save_index {
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
fn get_save_index(sector: Sector) -> u32 {
    LittleEndian::read_u32(&sector[0x0FFC..0x1000])
}
