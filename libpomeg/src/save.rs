use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryInto;

pub type Buffer = [u8; 0x20000];

#[derive(Copy, Clone)]
pub struct Save {
    pub sectors: [Sector; 0x20],
    pub slot_used: Option<SlotUsed>,
}

impl Save {
    fn new() -> Self {
        let sectors = [Sector::new(); 0x20];
        let slot_used = None;

        Save { sectors, slot_used }
    }

    pub fn from_buffer(buffer: Buffer) -> Self {
        let mut save = Save::new();

        for sector_id in 0..=31 {
            let offset = sector_id << 12;
            save.sectors[sector_id] = Sector::from_slice(&buffer[offset..offset + 0x1000]);
        }

        save.get_slot()
    }

    fn get_slot(mut self) -> Self {
        let slot_a = SlotStruct::from_slot(self.to_slot(0));
        let slot_b = SlotStruct::from_slot(self.to_slot(1));

        self.slot_used = SlotUsed::from_slots(slot_a, slot_b);

        self
    }

    pub fn to_slot(&self, slot_index: u8) -> Slot {
        let offset = match slot_index {
            0 => 0,
            1 => 14,
            _ => panic!("Slot cannot be greater than 1"),
        };

        self.sectors[offset..=offset + 13].try_into().unwrap()
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

    pub fn security_passed(&self) -> bool {
        const SECURITY_VALUE: u32 = 0x8012025;

        LittleEndian::read_u32(&self.0[0xFF8..=0xFFB]) == SECURITY_VALUE
    }
}

pub type Slot = [Sector; 14];

#[derive(Clone, Copy, Debug)]
pub enum SlotUsed {
    A = 0,
    B = 14,
}

impl SlotUsed {
    fn from_slots(slot_a: SlotStruct, slot_b: SlotStruct) -> Option<SlotUsed> {
        if slot_a.status == SlotStatus::Valid && slot_b.status == SlotStatus::Valid {
            if slot_a.counter == u32::MAX && slot_b.counter == 0
                || slot_b.counter == u32::MAX && slot_a.counter == 0
            {
                if slot_a.counter < slot_b.counter {
                    return Some(SlotUsed::A);
                }

                return Some(SlotUsed::B);
            }

            if slot_a.counter < slot_b.counter {
                return Some(SlotUsed::B);
            }

            return Some(SlotUsed::A);
        }

        if slot_a.status == SlotStatus::Valid {
            return Some(SlotUsed::A);
        }

        if slot_b.status == SlotStatus::Valid {
            return Some(SlotUsed::B);
        }

        None
    }
}

#[derive(Debug)]
struct SlotStruct {
    counter: u32,
    status: SlotStatus,
}

impl SlotStruct {
    /// Determines whether the slot status is empty, corrupt, or valid, along with the counter.
    ///
    /// If the security check never passes, the slot is empty.
    /// If all sectors pass the check, the slot is valid.
    /// Anything else should result in a corrupt slot.
    ///
    /// If no slots are valid, the save counter must be 0.
    /// Otherwise, the save counter value is derived from the last valid sector.
    fn from_slot(slot: Slot) -> Self {
        let mut slot_struct = Self {
            counter: 0,
            status: SlotStatus::Empty,
        };

        let mut checksums_passed = 0;

        // Iterate from the last sector to retrieve the last valid save counter
        for (loop_count, sector) in slot.iter().rev().enumerate() {
            // If this check passes, the slot cannot be empty
            if sector.security_passed() {
                slot_struct.status = SlotStatus::Corrupt;

                if sector.checksum_passed() {
                    // The counter should be determined by the last valid sector
                    if checksums_passed == 0 {
                        slot_struct.counter = sector.get_save_counter();
                    }

                    checksums_passed += 1;
                }
            }

            // Breaks loop if the counter is known and the slot is guaranteed to be corrupt
            if checksums_passed != 0 && checksums_passed != loop_count + 1 {
                break;
            }
        }

        if checksums_passed == 14 {
            slot_struct.status = SlotStatus::Valid;
        }

        slot_struct
    }
}

#[derive(Debug, PartialEq)]
pub enum SlotStatus {
    Empty = 0,
    Valid = 1,
    Corrupt = 2,
}
