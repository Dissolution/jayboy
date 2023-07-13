use crate::{get_rom_files, load_rom_memory};
use lib_jayboy::{
    Cartridge, FormatterBuilder, GBytes, InstructionReader, ReadOnlyMemory, TextBuilder,
};
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct RomScanner;
impl RomScanner {
    pub fn scan_roms() -> anyhow::Result<()> {
        const ENTRY_POINT_RANGE: RangeInclusive<u16> = (0x0100_u16..=0x0103_u16);
        let mut entry_points: HashSet<Box<[u8]>> = HashSet::new();

        let rom_files = get_rom_files()?;
        for rom_file in rom_files {
            let rom_bytes = load_rom_memory(rom_file)?;
            let got_bytes = rom_bytes.get_bytes(ENTRY_POINT_RANGE);
            match got_bytes {
                Ok(ep) => {
                    entry_points.insert(Box::from(ep));
                }
                Err(ex) => {
                    panic!("Oh shit: {}", ex);
                }
            }
        }

        // for output
        let mut k = entry_points
            .iter()
            .map(|ep| GBytes::from(ep.as_ref()))
            .collect::<Vec<GBytes>>();
        k.sort();

        let text: String = TextBuilder::build_string(|f| {
            f.append("All seen Entry Points:")
                .newline()
                .enumerate(k.iter(), |f, i, gbytes| {
                    let instructions = InstructionReader::as_instructions(gbytes.0);
                    f.enumerate(instructions.iter(), |f, i, instruction| {
                        f.debug(instruction).newline();
                    });
                });
            Ok(())
        });

        println!("{}", text);

        Ok(())
    }
}
