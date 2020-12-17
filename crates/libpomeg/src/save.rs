use std::convert::TryInto;

pub type Save = [Sector; 0x20];

impl FromBuffer for Save {}

pub trait FromBuffer {
    fn from_buffer(buffer: [u8; 0x20000]) -> Save {
        let mut save: Save = [[0; 0x1000]; 0x20];

        for sector_id in 0..31 {
            let offset = sector_id << 12;
            save[sector_id] = buffer[offset..offset + 0x1000].try_into().unwrap();
        }

        save
    }
}

pub type Sector = [u8; 0x1000];
