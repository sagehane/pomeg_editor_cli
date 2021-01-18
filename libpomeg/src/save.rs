use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryInto;

#[derive(Copy, Clone)]
pub struct Save(pub [Sector; 0x20]);

impl Save {
    fn new() -> Self {
        Save([Sector::new(); 0x20])
    }

    pub fn from_array(array: [u8; 0x20000]) -> Self {
        let mut save = Save::new();

        for sector_id in 0..=31 {
            let offset = sector_id << 12;
            save.0[sector_id] = Sector::from_slice(&array[offset..offset + 0x1000]);
        }

        save
    }

    pub fn to_slot(&self, slot_index: u8) -> Slot {
        let offset = match slot_index {
            0 => 0,
            1 => 14,
            _ => panic!("Slot cannot be greater than 1"),
        };

        self.0[offset..=offset + 13].try_into().unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Sector(pub [u8; 0x1000]);

impl Sector {
    fn new() -> Self {
        Sector([0; 0x1000])
    }

    fn from_slice(slice: &[u8]) -> Self {
        Sector(slice.try_into().unwrap())
    }

    /// Gets the save index from a sector
    pub fn get_save_counter(&self) -> u32 {
        LittleEndian::read_u32(&self.0[0x0FFC..=0x0FFF])
    }
}

pub type Slot = [Sector; 14];
