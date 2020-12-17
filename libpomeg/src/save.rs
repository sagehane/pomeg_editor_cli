use std::convert::TryInto;

pub trait DataStructure {
    fn new() -> Self;

    fn from_slice(slice: &[u8]) -> Self;
}

pub type Save = [Sector; 0x20];

impl DataStructure for Save {
    fn new() -> Self {
        [Sector::new(); 0x20]
    }

    fn from_slice(slice: &[u8]) -> Self {
        let mut save = Save::new();

        for sector_id in 0..32 {
            let offset = sector_id << 12;
            save[sector_id] = Sector::from_slice(&slice[offset..offset + 0x1000]);
        }

        save
    }
}

pub type Sector = [u8; 0x1000];

impl DataStructure for Sector {
    fn new() -> Self {
        [0; 0x1000]
    }

    fn from_slice(slice: &[u8]) -> Self {
        slice.try_into().unwrap()
    }
}
