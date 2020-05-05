/// The general purpose registers.
///
/// Chip-8 uses 16 registers named V0 to VF
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    VA = 0xA,
    VB = 0xB,
    VC = 0xC,
    VD = 0xD,
    VE = 0xE,
    VF = 0xF,
}

impl Register {
    /// Compares the register to another.
    ///
    /// Compares the register and returns either
    /// 1 if the register is greater than the provided one
    /// 0 if it is equal, or -1 if it is less than.
    ///
    ///### Arguments
    ///
    ///- **reg** : The register key we want to comapare.
    ///
    pub fn compare(&self, reg: Register) -> i8 {
        let left = *self as u8;
        let right = reg as u8;

        if left < right {
            -1
        } else if left == right {
            0
        } else {
            1
        }
    }

    /// Creates an iterator that iterates from V0 to the
    /// provided register.
    ///
    ///### Arguments
    ///
    ///- **to** : The last register in the range.
    ///
    pub fn iter_to(to: Register) -> RegisterIterator {
        RegisterIterator::new(to)
    }

    /// Gets the register from a byte address.
    ///
    pub fn new(byte: u8) -> Result<Self, String> {
        match byte {
            0x0 => Ok(Register::V0),
            0x1 => Ok(Register::V1),
            0x2 => Ok(Register::V2),
            0x3 => Ok(Register::V3),
            0x4 => Ok(Register::V4),
            0x5 => Ok(Register::V5),
            0x6 => Ok(Register::V6),
            0x7 => Ok(Register::V7),
            0x8 => Ok(Register::V8),
            0x9 => Ok(Register::V9),
            0xA => Ok(Register::VA),
            0xB => Ok(Register::VB),
            0xC => Ok(Register::VC),
            0xD => Ok(Register::VD),
            0xE => Ok(Register::VE),
            0xF => Ok(Register::VF),
            _ => Err(String::from("Invalid register")),
        }
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        self as usize
    }
}

/// An iterator that counts up from V0.
///
pub struct RegisterIterator {
    last: Register,
    current: Register,
}

impl RegisterIterator {
    /// Creates a new RegisterIterator.
    ///
    ///### Arguments
    ///
    ///- **last** : The last Register in the iteration.
    ///
    pub fn new(last: Register) -> Self {
        RegisterIterator {
            last,
            current: Register::V0,
        }
    }
}

impl Iterator for RegisterIterator {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.compare(self.last) == 1 {
            None
        } else {
            let cur = self.current;

            self.current = Register::new((cur as u8) + 1).unwrap();

            Some(cur)
        }
    }
}

#[test]
fn register_iterator_test() {
    use Register::*;

    let mut iter = RegisterIterator::new(V2);

    assert_eq!(iter.next(), Some(V0));
    assert_eq!(iter.next(), Some(V1));
    assert_eq!(iter.next(), Some(V2));
    assert_eq!(iter.next(), None);

    let mut iter = RegisterIterator::new(V0);

    assert_eq!(iter.next(), Some(V0));
    assert_eq!(iter.next(), None);
}

#[test]
fn register_new_generates_correct_register_name() {
    assert_eq!(Register::new(0x0).unwrap(), Register::V0);
    assert_eq!(Register::new(0x1).unwrap(), Register::V1);
    assert_eq!(Register::new(0x2).unwrap(), Register::V2);
    assert_eq!(Register::new(0x3).unwrap(), Register::V3);
    assert_eq!(Register::new(0x4).unwrap(), Register::V4);
    assert_eq!(Register::new(0x5).unwrap(), Register::V5);
    assert_eq!(Register::new(0x6).unwrap(), Register::V6);
    assert_eq!(Register::new(0x7).unwrap(), Register::V7);
    assert_eq!(Register::new(0x8).unwrap(), Register::V8);
    assert_eq!(Register::new(0x9).unwrap(), Register::V9);
    assert_eq!(Register::new(0xA).unwrap(), Register::VA);
    assert_eq!(Register::new(0xB).unwrap(), Register::VB);
    assert_eq!(Register::new(0xC).unwrap(), Register::VC);
    assert_eq!(Register::new(0xD).unwrap(), Register::VD);
    assert_eq!(Register::new(0xE).unwrap(), Register::VE);
    assert_eq!(Register::new(0xF).unwrap(), Register::VF);
}

#[test]
fn register_compare() {
    let reg1 = Register::V0;
    let reg2 = Register::V1;

    assert_eq!(reg1.compare(reg2), -1);
    assert_eq!(reg2.compare(reg1), 1);
    assert_eq!(reg1.compare(reg1), 0);
}
