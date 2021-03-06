pub use crate::save::Sector;
use byteorder::{ByteOrder, LittleEndian};

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

impl Sector {
    /// Checks if the checksum of a sector is valid by comparing the value of the two bytes stored in
    /// offset of 0xff4 with the value from `calculate_checksum`.
    pub fn checksum_passed(&self) -> bool {
        // Section ID cannot be 14 or above
        if self.section_id >= 14 {
            return false;
        }

        let calculated_checksum =
            calculate_checksum(&self.data[..SECTOR_SIZE[self.section_id as usize] as usize]);

        let checksum = LittleEndian::read_u16(&self.data[0xFF6..=0xFF7]);

        if calculated_checksum != checksum {
            return false;
        }

        true
    }
}

/// Separates a slice into litte-endian u32 values, gets the total sum of the slice, and gets the
/// sum of the upper and lower 16 bytes.
fn calculate_checksum(sector_slice: &[u8]) -> u16 {
    let mut checksum: u32 = 0;

    for i in (0..sector_slice.len()).step_by(4) {
        checksum = checksum.wrapping_add(LittleEndian::read_u32(&sector_slice[i..i + 4]));
    }

    ((checksum >> 16) as u16).wrapping_add(checksum as u16)
}
