use data::{Address, Byte, Nibble};

use model::Register;

type ParseResult = (Address, Register, Register, Byte, Nibble);

/// Parses the instruction and returns a tuple of the pieces of an instruction.
///
///### Arguments
///
///- **instr** : The raw instruction.
///
pub fn parse_raw_instruction(instr: u16) -> ParseResult {
    let (vx, vy) = get_registers(instr);
    (
        get_address(instr),
        vx,
        vy,
        get_byte(instr),
        get_nibble(instr),
    )
}

/// Gets the nibble from the instruction.
///
///### Arguments
///
///- **inst** : The raw instruction.
///
fn get_nibble(inst: u16) -> Nibble {
    Nibble::new((inst & 0x000F) as u8)
}

/// Gets the address from the instruction.
///
///### Arguments
///
///- **inst** : The raw instruction.
///
fn get_address(inst: u16) -> Address {
    (inst & 0x0FFF).into()
}

/// Gets the register keys from an instruction.
///
///### Arguments
///
///- **inst** : The raw instruction.
///
fn get_registers(inst: u16) -> (Register, Register) {
    let reg_bits = ((inst & 0x00F0) >> 4) as u8;
    let vy = Register::new(reg_bits);

    let reg_bits = ((inst & 0x0F00) >> 8) as u8;
    let vx = Register::new(reg_bits);

    if let (Ok(vx), Ok(vy)) = (vx, vy) {
        (vx, vy)
    } else {
        // Todo: Need to handle errors properly here.
        panic!("Error");
    }
}

/// Gets the byte value from an instruction.
///
///### Arguments
///
///- **inst** : The raw instruction.
///
fn get_byte(inst: u16) -> Byte {
    ((inst & 0x00FF) as u8).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_nibble_returns_the_correct_nibble() {
        let nib = get_nibble(0x1234);

        assert_eq!(nib, 0x4.into());
    }

    #[test]
    fn get_byte_returns_the_correct_byte() {
        let byte = get_byte(0x1234);

        assert_eq!(byte, 0x34.into());
    }

    #[test]
    fn get_address_returns_the_correct_address() {
        let address = get_address(0x1234);

        assert_eq!(address, 0x234.into());
    }

    #[test]
    fn get_registers_returns_the_correct_registers() {
        let (vx, vy) = get_registers(0x1234);

        assert_eq!(vx, Register::V2);
        assert_eq!(vy, Register::V3);
    }
}
