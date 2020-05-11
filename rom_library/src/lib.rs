use std::fs;
use std::io;

/// Load the rom with the given name
/// 
///### Arguments
/// 
///- **name** : The name of the rom.
///
pub fn load_file(name: String) -> Result<Vec<u8>, io::Error> {
    fs::read(format!("{}.ch8", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_file() {
        let res = load_file("test_opcode".into());

        assert!(res.is_ok());
    }
}
