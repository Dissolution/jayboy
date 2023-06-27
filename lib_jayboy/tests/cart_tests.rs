use anyhow::anyhow;
use lib_jayboy::*;

pub const NINTENDO_LOGO_BYTES: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub fn validate_nintendo_logo(cart: &Cartridge) -> anyhow::Result<()> {
    if cart.nintendo_logo().eq(&NINTENDO_LOGO_BYTES) {
        Ok(())
    } else {
        Err(anyhow!("Invalid Nintendo Logo"))
    }
}

pub fn validate_ram_size_vs_cartridge_type(cart: &Cartridge) -> anyhow::Result<()> {
    // If the cartridge type does not include “RAM” in its name, this should be set to 0.
    // This includes MBC2, since its 512 × 4 bits of memory are built directly into the mapper.
    let has_ram = cart.cartridge_type().ram;
    let ram_size = cart.ram_size();
    if has_ram && ram_size == 0 {
        Err(anyhow!("Cartridge Type indicates RAM, but RAM size is 0"))
    } else if !has_ram && ram_size != 0 {
        Err(anyhow!(
            "Cartridge Type indicates no RAM, but RAM size is {}",
            ram_size
        ))
    } else {
        Ok(())
    }
}

/// The BOOT ROM computes the header checksum as follows:
/// ```C
/// uint8_t checksum = 0;
/// for (uint16_t address = 0x0134; address <= 0x014C; address++) {
///     checksum = checksum - rom[address] - 1;
/// }
/// ```
pub fn generate_header_checksum(cart: &Cartridge) -> u8 {
    let mut checksum: u8 = 0;
    let bytes = cart.get_bytes();
    assert!(bytes.len() >= 0x014C);
    for address in 0x0134..=0x014C_u16 {
        let byte = bytes[address as usize];
        checksum = u8::wrapping_sub(checksum, byte);
        checksum = u8::wrapping_sub(checksum, 1);
    }
    checksum
}

/// These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes).
pub fn generate_global_checksum(cart: &Cartridge) -> u16 {
    let mut checksum: u16 = 0;
    let bytes = cart.get_bytes();
    assert!(bytes.len() >= 0x14D);
    // TODO: Is this for the entire ROM (as in _every_ other byte) or just this range below?
    for byte in bytes[0x0100..=0x14D].iter() {
        checksum = u16::wrapping_add(checksum, *byte as u16);
    }
    checksum
}

pub fn validate_cart(cart: &Cartridge) -> anyhow::Result<()> {
    // validate the logo
    validate_nintendo_logo(cart)?;
    validate_ram_size_vs_cartridge_type(cart)?;
    let checksum = generate_header_checksum(cart);
    if checksum != cart.header_checksum() {
        return Err(anyhow!("Invalid Header Checksum"));
    }
    let checksum = generate_global_checksum(cart);
    if checksum != cart.global_checksum() {
        return Err(anyhow!("Invalid Global Checksum"));
    }
    // more?
    Ok(())
}
