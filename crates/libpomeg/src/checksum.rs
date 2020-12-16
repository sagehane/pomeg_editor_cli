use crate::sector_by_id;
use byteorder::{ByteOrder, LittleEndian};

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

/// Checks if the save file has the correct checksum. Currently only checks through sectors 0 to
/// 27, leaving 28 to 31 unchecked.
pub fn is_valid_checksum(buffer: &[u8; 0x20000]) -> bool {
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
    let section_id = *&sector[0xFF4] as usize;

    if section_id == 0xFF {
        println!("Checksum skipped");

        return true;
    }

    let calculated_checksum = calculate_checksum(&sector[..SECTOR_SIZE[section_id] as usize]);

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
