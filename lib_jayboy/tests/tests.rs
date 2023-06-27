pub use lib_jayboy::*;
use std::fs;
use std::path::PathBuf;

mod cart_tests;

fn get_rom_files() -> Vec<PathBuf> {
    let path = r"c:\gb_roms\";

    let gb_rom_paths = fs::read_dir(path)
        .unwrap()
        .filter_map(|r| r.ok())
        .map(|de| de.path())
        .filter(|p| {
            if let Some(ext) = p.extension() {
                ext == "gb" // .gb is the extension for Game Boy rom files
            } else {
                false
            }
        })
        .collect();
    gb_rom_paths
}

#[test]
pub fn test_all_roms() {
    let gb_rom_paths = get_rom_files();
    for file in gb_rom_paths.iter() {
        println!("Testing {:?}", file);
        let load_cart = Roms::load_cartridge(file);
        println!("Loaded: {:?}", load_cart);
        if let Ok(cart) = load_cart {
            println!("Loaded Cart {}", cart.title());
            let tests = cart_tests::validate_cart(&cart);
            println!("Cart Tests: {:?}", tests);
            println!("DEBUG");
        }

        //let cart = Roms::load_cartridge(file).unwrap();
        //cart_tests::validate_cart(&cart).unwrap();
        //println!("Validated \"{}\"", cart.title());
    }
}
