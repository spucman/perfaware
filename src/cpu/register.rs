use std::fmt::{Display, Formatter, Result};

// AL W=0 / AX W=1
const REG_FIELD_A: u8 = 0b000;
// CL W=0 / CX W=1
const REG_FIELD_C: u8 = 0b001;
// DL W=0 / DX W=1
const REG_FIELD_D: u8 = 0b010;
// BL W=0 / BX W=1
const REG_FIELD_B: u8 = 0b011;
// AH W=0 / SP W=1
const REG_FIELD_AH: u8 = 0b100;
// CH W=0 / BP W=1
const REG_FIELD_CH: u8 = 0b101;
// DH W=0 / SI W=1
const REG_FIELD_DH: u8 = 0b110;
// BH W=0 / DI W=1
const REG_FIELD_BH: u8 = 0b111;

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

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AL => write!(f, "AL"),
            AX => write!(f, "AX"),
            CL => write!(f, "CL"),
            CX => write!(f, "CX"),
            DL => write!(f, "DL"),
            DX => write!(f, "DX"),
            BL => write!(f, "BL"),
            BX => write!(f, "BX"),
            AH => write!(f, "AH"),
            SP => write!(f, "SP"),
            CH => write!(f, "CH"),
            BP => write!(f, "BP"),
            DH => write!(f, "DH"),
            SI => write!(f, "SI"),
            BH => write!(f, "BH"),
            DI => write!(f, "DI"),
        }
    }
}

enum Instruction {
    MOV,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MOV => write!(f, "MOV"),
        }
    }
}

struct Command {
    instruction: Instruction,
    source: Register,
    destination: Register,
}

impl From<&[u8]> for Command {
    fn from(item: &[u8]) -> Command {}
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
