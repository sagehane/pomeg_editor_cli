const SECTOR_SIZE: [u16; 14] = [
    3884, 3968, 3968, 3968, 3848, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 3968, 2000,
];

fn calculate_checksum(sector: &[u8]) -> u16 {
    let mut checksum: u32 = 0;
    let mut shift = 0;

    for b in sector.iter() {
        checksum = checksum.wrapping_add((*b as u32) << 8 * shift);
        shift = (shift + 1) % 4;
    }

    ((checksum >> 16) as u16)
        .wrapping_add(checksum as u16)
        .rotate_right(8)
}

fn check_sector(sector_id: u8, content: &Vec<u8>) -> bool {
    let mut is_valid = false;

    let calculated_checksum = calculate_checksum(
        &content[(sector_id as usize) << 12
            ..((sector_id as usize) << 12)
                + SECTOR_SIZE[*&content[((sector_id as usize) << 12) + 0xff4] as usize] as usize],
    );

    let checksum = ((content[((sector_id as usize) << 12) + 0xff6] as u16) << 8)
        + content[((sector_id as usize) << 12) + 0xff7] as u16;

    if calculated_checksum == checksum {
        is_valid = true;
    }

    is_valid
}

pub fn check_save(content: &Vec<u8>) -> bool {
    if content.len() != 2_usize.pow(17) {
        return false;
    }

    for i in 0..28 {
        if check_sector(i, content) == false {
            return false;
        }
    }

    true
}
