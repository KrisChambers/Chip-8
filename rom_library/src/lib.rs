mod roms;
use std::io;
use roms::{ TETRIS, BREAKOUT, INVADERS };

/// Load the rom with the given name
///
///### Arguments
///
///- **name** : The name of the rom.
///
pub fn load_rom(name: String) -> Result<Vec<u8>, io::Error> {
    let name: &str = &name.to_lowercase();
    
    match name {
        "tetris" => Ok(Vec::from(TETRIS)),
        "breakout" => Ok(Vec::from(BREAKOUT)),
        "invaders" => Ok(Vec::from(INVADERS)),
        _ => Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find rom for {}", name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_file() {
        let res = load_rom("tetris".into());

        assert!(res.is_ok());
    }
}
