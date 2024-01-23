use std::fmt::{Display, Formatter, Result};

pub enum Register {
    AL,
    AX,
    CL,
    CX,
    DL,
    DX,
    BL,
    BX,
    AH,
    SP,
    CH,
    BP,
    DH,
    SI,
    BH,
    DI,
}

impl Register {
    pub fn from_reg(item: u8, w: bool) -> Register {
        let reg = if w { item | 0b1000 } else { item };
        match reg {
            0b0000 => Register::AL,
            0b1000 => Register::AX,
            0b0001 => Register::CL,
            0b1001 => Register::CX,
            0b0010 => Register::DL,
            0b1010 => Register::DX,
            0b0011 => Register::BL,
            0b1011 => Register::BX,
            0b0100 => Register::AH,
            0b1100 => Register::SP,
            0b0101 => Register::CH,
            0b1101 => Register::BP,
            0b0110 => Register::DH,
            0b1110 => Register::SI,
            0b0111 => Register::BH,
            0b1111 => Register::DI,
            _ => panic!("register not found"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Register::AL => write!(f, "al"),
            Register::AX => write!(f, "ax"),
            Register::CL => write!(f, "cl"),
            Register::CX => write!(f, "cx"),
            Register::DL => write!(f, "dl"),
            Register::DX => write!(f, "dx"),
            Register::BL => write!(f, "bl"),
            Register::BX => write!(f, "bx"),
            Register::AH => write!(f, "ah"),
            Register::SP => write!(f, "sp"),
            Register::CH => write!(f, "ch"),
            Register::BP => write!(f, "bp"),
            Register::DH => write!(f, "dh"),
            Register::SI => write!(f, "si"),
            Register::BH => write!(f, "bh"),
            Register::DI => write!(f, "di"),
        }
    }
}

pub struct Memory {
    pub registers: Option<Vec<Register>>,
    pub displacements: Option<(u8, Option<u8>)>,
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match (self.registers, self.displacements) {
            (Some(r), Some((dh, dl))) => write!(
                f,
                "{} + {}",
                r.iter()
                    .map(|v| v.to_string())
                    .reduce(|acc, s| format!("{} + {}", acc, s))
                    .unwrap_or_default(),
                dl.map(|v| ((dh as u16) << 8) | (v as u16))
                    .unwrap_or(dh as u16)
            ),
            (Some(r), None) => write!(
                f,
                "{}",
                r.iter()
                    .map(|v| v.to_string())
                    .reduce(|acc, s| format!("{} + {}", acc, s))
                    .unwrap_or_default()
            ),
            (None, Some((dh, dl))) => write!(
                f,
                "{}",
                dl.map(|v| ((dh as u16) << 8) | (v as u16))
                    .unwrap_or(dh as u16)
            ),
            (None, None) => write!(f, "should not happen"),
        }
    }
}

pub enum Storage {
    Mem(Memory),
    Reg(Register),
    Data(u8, Option<u8>),
}

impl Display for Storage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Storage::Mem(v) => v.fmt(f),
            Storage::Reg(v) => v.fmt(f),
            Storage::Data(dh, dl) => write!(
                f,
                "{}",
                dl.map(|v| ((*dh as u16) << 8) | (v as u16))
                    .unwrap_or(*dh as u16)
            ),
        }
    }
}

#[cfg(test)]
mod tests {}
