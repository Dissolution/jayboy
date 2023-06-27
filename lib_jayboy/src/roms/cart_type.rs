use anyhow::anyhow;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CartridgeType {
    pub mbc: u8,
    pub ram: bool,
    pub battery: bool,
    pub rom: bool,
    pub mmm01: bool,
    pub timer: bool,
    pub rumble: bool,
    pub sensor: bool,
    pub camera: bool,
    pub bandai_tama5: bool,
    pub huc: u8,
}
impl CartridgeType {}

impl TryFrom<u8> for CartridgeType {
    type Error = anyhow::Error;

    // TODO: Pan Docs says I need to watch for Pocket Monsters: Crystal Version
    // to know that MBC3 might be MCB30
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // ROM ONLY
            0x00 => Ok(CartridgeType {
                rom: true,
                ..Default::default()
            }),
            0x01 => Ok(CartridgeType {
                mbc: 1,
                ..Default::default()
            }),
            0x02 => Ok(CartridgeType {
                mbc: 1,
                ram: true,
                ..Default::default()
            }),
            0x03 => Ok(CartridgeType {
                mbc: 1,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x04 => DNE
            0x05 => Ok(CartridgeType {
                mbc: 2,
                ..Default::default()
            }),
            0x06 => Ok(CartridgeType {
                mbc: 2,
                battery: true,
                ..Default::default()
            }),
            // 0x07 => DNE
            // This cart type should not exist (no licensed cart does)
            0x08 => Ok(CartridgeType {
                rom: true,
                ram: true,
                ..Default::default()
            }),
            // This cart type should not exist (no licensed cart does)
            0x09 => Ok(CartridgeType {
                rom: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x0A => DNE
            0x0B => Ok(CartridgeType {
                mmm01: true,
                ..Default::default()
            }),
            0x0C => Ok(CartridgeType {
                mmm01: true,
                ram: true,
                ..Default::default()
            }),
            0x0D => Ok(CartridgeType {
                mmm01: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x0E => DNE
            0x0F => Ok(CartridgeType {
                mbc: 3,
                timer: true,
                battery: true,
                ..Default::default()
            }),
            0x10 => Ok(CartridgeType {
                mbc: 3,
                timer: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            0x11 => Ok(CartridgeType {
                mbc: 3,
                ..Default::default()
            }),
            0x12 => Ok(CartridgeType {
                mbc: 3,
                ram: true,
                ..Default::default()
            }),
            0x13 => Ok(CartridgeType {
                mbc: 3,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x14 -> 0x18 => DNE
            0x19 => Ok(CartridgeType {
                mbc: 5,
                ..Default::default()
            }),
            0x1A => Ok(CartridgeType {
                mbc: 5,
                ram: true,
                ..Default::default()
            }),
            0x1B => Ok(CartridgeType {
                mbc: 5,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            0x1C => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ..Default::default()
            }),
            0x1D => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ram: true,
                ..Default::default()
            }),
            0x1E => Ok(CartridgeType {
                mbc: 5,
                rumble: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x1F => DNE
            0x20 => Ok(CartridgeType {
                mbc: 6,
                ..Default::default()
            }),
            // 0x21 => DNE
            0x22 => Ok(CartridgeType {
                mbc: 7,
                sensor: true,
                rumble: true,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            // 0x23 -> 0xFB => DNE
            0xFC => Ok(CartridgeType {
                camera: true,
                ..Default::default()
            }),
            0xFD => Ok(CartridgeType {
                bandai_tama5: true,
                ..Default::default()
            }),
            0xFE => Ok(CartridgeType {
                huc: 3,
                ..Default::default()
            }),
            0xFF => Ok(CartridgeType {
                huc: 1,
                ram: true,
                battery: true,
                ..Default::default()
            }),
            _ => Err(anyhow!("Unknown cartridge type: {}", value)),
        }
    }
}
