use std::fs;
use std::io;
//use std::env;

/// Load the rom with the given name
///
///### Arguments
///
///- **name** : The name of the rom.
///
pub fn load_rom(name: String) -> Result<Vec<u8>, io::Error> {
    let file = format!("rom_library/{}.ch8", name);
    fs::read(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_file() {
        let res = load_rom("test_opcode".into());

        assert!(res.is_ok());
    }
}
