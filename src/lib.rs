use byteorder::{ByteOrder, LittleEndian};

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

/// Separates a slice into litte-endian u32 values, gets the total sum of the slice, and gets the
/// sum of the upper and lower 16 bytes.
fn calculate_checksum(sector: &[u8]) -> u16 {
    let mut checksum: u32 = 0;

    for i in (0..sector.len()).step_by(4) {
        checksum = checksum.wrapping_add(LittleEndian::read_u32(&sector[i..i + 4]));
    }

    ((checksum >> 16) as u16).wrapping_add(checksum as u16)
}

/// Checks if the checksum of a sector is valid by comparing the value of the two bytes stored in
/// offset of 0xff4 with the value from `calculate_checksum`.
fn is_valid_sector(sector_id: u8, content: &Vec<u8>) -> bool {
    let sector_offset: usize = (sector_id as usize) << 12;

    let calculated_checksum = calculate_checksum(
        &content[sector_offset
            ..sector_offset + SECTOR_SIZE[*&content[(sector_offset) + 0xff4] as usize] as usize],
    );

    let checksum = LittleEndian::read_u16(
        &content[(sector_offset) + 0xff6 as usize..(sector_offset) + 0xff8 as usize],
    );

    if calculated_checksum != checksum {
        return false;
    }

    true
}

/// Checks if the save file has the correct checksum. Currently only checks through sectors 0 to
/// 28, leaving 29 to 31 unchecked.
pub fn is_valid_save(content: &Vec<u8>) -> bool {
    if content.len() != 2_usize.pow(17) {
        eprintln!("The savefile is not 128 KiB in size");

        return false;
    }

    for i in 0..28 {
        if !is_valid_sector(i, content) {
            eprintln!("Sector {} has an invalid checksum", i);

            return false;
        }
    }

    true
}
