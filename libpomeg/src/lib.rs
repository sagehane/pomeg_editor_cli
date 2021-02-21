use std::convert::TryInto;

use crate::checksum::Sector;
use crate::encoding::slice_to_string;
pub use crate::save::{Save, Slot, SlotStatus, SlotUsed};

use byteorder::{ByteOrder, LittleEndian};

mod checksum;
mod encoding;
mod save;

#[derive(Debug)]
pub struct SaveStruct {
    slot_used: SlotUsed,
    trainer: Trainer,
    gender: Gender,
}

impl SaveStruct {
    pub fn from_save(save: Save) -> Self {
        let slot_used = save.slot_used.unwrap();

        let trainer = Trainer::from_sector(&save.sectors[slot_used as usize + 1]);

        let gender = Gender::from_sector(&save.sectors[slot_used as usize + 1]);

        Self {
            slot_used,
            trainer,
            gender,
        }
    }
}

struct Trainer {
    name: [u8; 7],
    public: u16,
    secret: u16,
}

impl Trainer {
    fn from_sector(sector: &Sector) -> Self {
        let name = sector.0[0..=6].try_into().unwrap();
        let public = LittleEndian::read_u16(&sector.0[0xA..=0xB]);
        let secret = LittleEndian::read_u16(&sector.0[0xD..=0xE]);

        Trainer {
            name,
            public,
            secret,
        }
    }
}

impl std::fmt::Debug for Trainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trainer")
            .field("name", &slice_to_string(&self.name))
            .field("public", &format!("{:05}", self.public))
            .field("secret", &format!("{:05}", self.secret))
            .finish()
    }
}

#[derive(Debug)]
enum Gender {
    Boy = 0,
    Girl = 1,
}

impl Gender {
    fn from_sector(sector: &Sector) -> Self {
        let gender = sector.0[0x8];

        return match gender {
            0 => Gender::Boy,
            1 => Gender::Girl,
            _ => panic!("Gender should be 0 or 1"),
        };
    }
}
