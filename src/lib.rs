use byteorder::{ByteOrder, LittleEndian};

const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

/// Take an buffer and id to return a sector in the form of a slice
///
/// # Panics
///
/// Panics when `sector_id > 31`
fn buffer_to_sector(sector_id: u8, buffer: &[u8]) -> &[u8] {
    if sector_id > 31 {
        panic!("Sector must be between 0 and 31")
    }

    let sector_offset: usize = (sector_id as usize) << 12;

    &buffer[sector_offset..sector_offset + 0x1000]
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

fn get_save_index(sector: &[u8]) -> u32 {
    LittleEndian::read_u32(&sector[0x0FFC..0x1000])
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

/// Checks if the save file has the correct checksum. Currently only checks through sectors 0 to
/// 27, leaving 28 to 31 unchecked.
pub fn is_valid_save(buffer: &[u8]) -> bool {
    let mut save_index: u32 = get_save_index(buffer_to_sector(0, buffer));
    let mut is_slot_a = true;

    for sector_id in 0..27 {
        if sector_id == 14 {
            let retrieved_index = get_save_index(buffer_to_sector(sector_id, buffer));

            if retrieved_index == save_index {
                eprintln!("Slot A and B has the same save_index");

                return false;
            } else if retrieved_index > save_index {
                is_slot_a = false;
            }

            save_index = retrieved_index;
        } else if sector_id != 0
            && save_index != get_save_index(buffer_to_sector(sector_id, buffer))
        {
            eprintln!(
                "Sector {} has an invalid save_index, expected \"{}\" but got \"{}\"",
                sector_id,
                save_index,
                get_save_index(buffer_to_sector(sector_id, buffer))
            );

            return false;
        }

        if !is_valid_sector(buffer_to_sector(sector_id, buffer)) {
            eprintln!("Sector {} has an invalid checksum", sector_id);

            return false;
        }
    }

    match is_slot_a {
        true => println!("Save is stored in slot A"),
        false => println!("Save is stored in slot B"),
    }

    true
}
