use anyhow::anyhow;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CartridgeType {
    pub byte: u8,
    pub mbc: u8,
    pub ram: bool,
    pub battery: bool,
    pub rom: bool,
    pub mmm01: bool,
    pub timer: bool,
    pub rumble: bool,
    pub sensor: bool,
    pub camera: bool,
    pub tamagochi: bool,
    pub huc: u8,
    pub bootleg: bool,
}
impl CartridgeType {}

impl Display for CartridgeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.bootleg {
            return write!(f, "BOOTLEG");
        }

        write!(f, "0x{:0<2X}: ", self.byte)?;
        write!(f, "MBC{}", self.mbc)?;
        if self.ram {
            write!(f, "+RAM")?;
        }
        if self.battery {
            write!(f, "+BATTERY")?;
        }
        if self.rom {
            write!(f, "+ROM")?;
        }
        if self.mmm01 {
            write!(f, "+MMM01")?;
        }
        if self.timer {
            write!(f, "+TIMER")?;
        }
        if self.rumble {
            write!(f, "+RUMBLE")?;
        }
        if self.sensor {
            write!(f, "+SENSOR")?;
        }
        if self.camera {
            write!(f, "+CAMERA")?;
        }
        if self.tamagochi {
            write!(f, "+TAMAGOCHI")?;
        }
        if self.huc > 0 {
            write!(f, "+HUC{}", self.huc)?;
        }

        FmtResult::Ok(())
    }
}

impl TryFrom<u8> for CartridgeType {
    type Error = anyhow::Error;

    // TODO: Pan Docs says I need to watch for Pocket Monsters: Crystal Version
    // to know that MBC3 might be MCB30
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let mut cart_type = CartridgeType {
            byte: value,
            ..Default::default()
        };

        match value {
            // ROM ONLY
            0x00 => {
                cart_type.rom = true;
                Ok(cart_type)
            }
            0x01 => {
                cart_type.mbc = 1;
                Ok(cart_type)
            }
            0x02 => {
                cart_type.mbc = 1;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x03 => {
                cart_type.mbc = 1;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x04 => DNE
            0x05 => {
                cart_type.mbc = 2;
                Ok(cart_type)
            }
            0x06 => {
                cart_type.mbc = 2;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x07 => DNE
            // This cart type should not exist (no licensed cart does)
            0x08 => {
                cart_type.rom = true;
                cart_type.ram = true;
                Ok(cart_type)
            }
            // This cart type should not exist (no licensed cart does)
            0x09 => {
                cart_type.rom = true;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x0A => DNE
            0x0B => {
                cart_type.mmm01 = true;
                Ok(cart_type)
            }
            0x0C => {
                cart_type.mmm01 = true;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x0D => {
                cart_type.mmm01 = true;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x0E => DNE
            0x0F => {
                cart_type.mbc = 3;
                cart_type.timer = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            0x10 => {
                cart_type.mbc = 3;
                cart_type.timer = true;
                cart_type.battery = true;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x11 => {
                cart_type.mbc = 3;
                Ok(cart_type)
            }
            0x12 => {
                cart_type.mbc = 3;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x13 => {
                cart_type.mbc = 3;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x14 -> 0x18 => DNE
            0x19 => {
                cart_type.mbc = 5;
                Ok(cart_type)
            }
            0x1A => {
                cart_type.mbc = 5;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x1B => {
                cart_type.mbc = 5;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            0x1C => {
                cart_type.mbc = 5;
                cart_type.rumble = true;
                Ok(cart_type)
            }
            0x1D => {
                cart_type.mbc = 5;
                cart_type.rumble = true;
                cart_type.ram = true;
                Ok(cart_type)
            }
            0x1E => {
                cart_type.mbc = 5;
                cart_type.rumble = true;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x1F => DNE
            0x20 => {
                cart_type.mbc = 6;
                Ok(cart_type)
            }
            // 0x21 => DNE
            0x22 => {
                cart_type.mbc = 7;
                cart_type.sensor = true;
                cart_type.rumble = true;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            // 0x23 -> 0xFB => DNE
            0xFC => {
                cart_type.camera = true;
                // TODO: This was not true in the original documentation, but follows per .gb roms, verify!
                cart_type.ram = true;
                Ok(cart_type)
            }
            0xFD => {
                cart_type.tamagochi = true;
                Ok(cart_type)
            }
            0xFE => {
                cart_type.huc = 3;
                Ok(cart_type)
            }
            0xFF => {
                cart_type.huc = 1;
                cart_type.ram = true;
                cart_type.battery = true;
                Ok(cart_type)
            }
            _ => Err(anyhow!("Unknown cartridge type: 0x{:0<2X}", value)),
        }
    }
}
