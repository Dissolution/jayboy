use crate::{get_rom_files, load_rom_memory};
use lib_jayboy::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct RomScanner;
impl RomScanner {
    pub fn scan_titles() -> Vec<Box<str>> {
        let mut titles = Vec::new();
        let rom_files = get_rom_files().unwrap();
        for rom_file in rom_files {
            let cart = Cartridge::load_from(&rom_file).unwrap();
            let title = cart.title().unwrap_or(gb_str::empty());
            titles.push(Box::from(title.to_string()));
        }
        titles
    }

    pub fn scan_entry_points() -> anyhow::Result<()> {
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

        // TODO: Fix this:
        // // for output
        // let mut entry_points = entry_points.into_iter().collect::<Vec<Box<[u8]>>>();
        // entry_points.sort();
        //
        // let text: String = TextBuilder::build_string(|f| {
        //     f.append("All seen Entry Points:").newline().enumerate(
        //         entry_points.iter(),
        //         |f, i, gbytes| {
        //             println!("{} instructions", gbytes.0.len());
        //             let instructions = InstructionReader::read_instructions(gbytes.0).unwrap();
        //             f.enumerate(instructions.iter(), |f, i, instruction| {
        //                 f.debug(instruction).newline();
        //             });
        //             println!();
        //         },
        //     );
        //     Ok(())
        // });
        //
        // println!("{}", text);

        Ok(())
    }
}
