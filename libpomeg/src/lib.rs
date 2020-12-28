use crate::checksum::is_valid_sector;
use crate::encoding::slice_to_string;
pub use crate::save::{DataStructure, Save, Sector, Slot, ToSlot};
use byteorder::{ByteOrder, LittleEndian};

mod checksum;
mod encoding;
mod save;

const SECURITY_VALUE: u32 = 0x8012025;

#[derive(Debug)]
pub struct SaveStruct {
    slot_info: SlotInfo,
    trainer_id: TrainerID,
    trainer_name: String,
}

impl SaveStruct {
    pub fn from_save(save: Save) -> Self {
        let slot_info = SlotInfo::from_save(save);

        let trainer_id = TrainerID::from_sector(save[slot_info.slot_used.unwrap() as usize + 1]);

        let trainer_name = slice_to_string(&save[slot_info.slot_used.unwrap() as usize + 1][0..=6]);

        Self {
            slot_info,
            trainer_id,
            trainer_name,
        }
    }
}

#[derive(Debug)]
struct SlotInfo {
    slot_used: Option<SaveSlot>,
    slot_a: SlotStruct,
    slot_b: SlotStruct,
}

impl SlotInfo {
    /// Find out the slot from given save
    fn from_save(save: Save) -> Self {
        let slot_a = SlotStruct::from_slot(save.to_slot_a());
        let slot_b = SlotStruct::from_slot(save.to_slot_b());

        let slot_used = SlotInfo::get_slot(&slot_a, &slot_b);

        Self {
            slot_used,
            slot_a,
            slot_b,
        }
    }

    fn get_slot(slot_a: &SlotStruct, slot_b: &SlotStruct) -> Option<SaveSlot> {
        if slot_a.status == SaveStatus::Valid && slot_b.status == SaveStatus::Valid {
            if slot_a.counter == u32::MAX && slot_b.counter == 0
                || slot_b.counter == u32::MAX && slot_a.counter == 0
            {
                if slot_a.counter < slot_b.counter {
                    return Some(SaveSlot::A);
                }

                return Some(SaveSlot::B);
            }

            if slot_a.counter < slot_b.counter {
                return Some(SaveSlot::B);
            }

            return Some(SaveSlot::A);
        }

        if slot_a.status == SaveStatus::Valid {
            return Some(SaveSlot::A);
        }

        if slot_b.status == SaveStatus::Valid {
            return Some(SaveSlot::B);
        }

        None
    }
}

#[derive(Debug)]
struct SlotStruct {
    counter: u32,
    status: SaveStatus,
}

impl SlotStruct {
    fn from_slot(slot: Slot) -> Self {
        let mut passed_checksum = 0;

        let mut slot_struct = Self {
            counter: 0,
            status: SaveStatus::Empty,
        };

        let mut security_passed = false;

        for sector in slot.iter() {
            if LittleEndian::read_u32(&sector[0xFF8..=0xFFB]) == SECURITY_VALUE {
                security_passed = true;

                if is_valid_sector(sector) {
                    slot_struct.counter = get_save_counter(sector);
                    passed_checksum += 1;
                }
            }
        }

        if security_passed {
            if passed_checksum == slot.len() {
                slot_struct.status = SaveStatus::Valid;
            } else {
                slot_struct.status = SaveStatus::Corrupt;
            }
        }

        slot_struct
    }
}

#[derive(Debug, Clone, Copy)]
enum SaveSlot {
    A = 0,
    B = 14,
}

#[derive(Debug, PartialEq)]
enum SaveStatus {
    Empty = 0,
    Valid = 1,
    Corrupt = 2,
}

#[derive(Debug)]
struct TrainerID {
    public: u16,
    secret: u16,
}

impl TrainerID {
    fn from_sector(sector: Sector) -> Self {
        let public = LittleEndian::read_u16(&sector[0xA..=0xB]);
        let secret = LittleEndian::read_u16(&sector[0xD..=0xE]);

        TrainerID { public, secret }
    }
}

/// Gets the save index from a sector
fn get_save_counter(sector: &Sector) -> u32 {
    LittleEndian::read_u32(&sector[0x0FFC..=0x0FFF])
}
