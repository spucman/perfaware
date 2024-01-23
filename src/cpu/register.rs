use std::fmt::{Display, Formatter, Result};

enum Register {
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
    fn from_reg(item: u8, w: bool) -> Register {
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

#[derive(PartialEq)]
enum MovVariant {
    ToFromReg,
    ImmediateToStorage,
    ImmediateToReg,
    MemToAcc,
    AccToMem,
}

#[derive(PartialEq)]
enum Instruction {
    Mov(MovVariant),
    Nan,
}

impl From<u8> for Instruction {
    fn from(item: u8) -> Instruction {
        match item << 4 {
            0b1011 => Instruction::Mov(MovVariant::ImmediateToReg),
            _ => match item << 2 {
                0b100010 => Instruction::Mov(MovVariant::ToFromReg),
                _ => match item << 1 {
                    0b1100011 => Instruction::Mov(MovVariant::ImmediateToStorage),
                    0b1010000 => Instruction::Mov(MovVariant::MemToAcc),
                    0b1010001 => Instruction::Mov(MovVariant::AccToMem),
                    _ => Instruction::Nan,
                },
            },
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Instruction::Mov(_) => write!(f, "mov"),
            Instruction::Nan => write!(f, "n/a"),
        }
    }
}

struct Memory {
    registers: Option<Vec<Register>>,
    displacements: Option<(u8, Option<u8>)>,
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

enum Storage {
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

pub struct Command {
    instruction: Instruction,
    source: Storage,
    destination: Storage,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            instruction: Instruction::Nan,
            source: Storage::Reg(Register::AX),
            destination: Storage::Reg(Register::AX),
        }
    }
}

enum CmdParsingState {
    First,
    Hi,
    Lo,
}

impl From<&[u8]> for Command {
    fn from(items: &[u8]) -> Vec<Command> {
        let mut res = Vec::new();

        let mut state = CmdParsingState::First;
        let mut cur_instr = Instruction::Nan;
        for item in items {
            match state {
                First => {
                    if cur_instr != Instruction::Nan {
                        //TODO create command here
                    }
                    cur_instr = Instruction::from(*item);
                    match cur_instr {
                        Instruction::Mov(m) => match m {
                            MovVariant::AccToMem | MovVariant::MemToAcc => {}
                            MovVariant::ToFromReg => {}
                            MovVariant::ImmediateToReg => {}
                            MovVariant::ImmediateToStorage => {}
                        },
                        Instruction::Nan => {
                            println!("Invalid instruction found");
                            return Vec::default();
                        }
                    }
                }
            }
        }

        let first = items[0];
        let instr = Instruction::from(first >> 2);
        let to_reg = (first & 0b00000010) == 0b00000010;
        let w = (first & 0b00000001) == 0b00000001;

        let second = items[1];
        if (second & 0b11000000) != 0b11000000 {
            println!("Only Register Mode is allowed right now");
            return res;
        }

        let (dest, source) = if to_reg {
            ((second & 0b00111000) >> 3, second & 0b00000111)
        } else {
            (second & 0b00000111, (second & 0b00111000) >> 3)
        };

        Command {
            instruction: instr,
            source: Register::from_reg(source, w),
            destination: Register::from_reg(dest, w),
        };

        res
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {}, {}",
            self.instruction, self.destination, self.source
        )
    }
}

#[cfg(test)]
mod tests {}
