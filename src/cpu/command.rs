use crate::cpu::storage::{Memory, Register, Storage};

use std::fmt::{Display, Formatter, Result};

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

pub struct Command {
    pub instruction: Instruction,
    pub source: Storage,
    pub destination: Storage,
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

#[derive(PartialEq)]
enum Mode {
    MemNoDisplacement,
    Mem8BitDisplacement,
    Mem16BitDisplacement,
    Reg,
}

impl From<u8> for Mode {
    fn from(item: u8) -> Mode {
        match item {
            0b00000000 => Mode::MemNoDisplacement,
            0b00000001 => Mode::Mem8BitDisplacement,
            0b00000010 => Mode::Mem16BitDisplacement,
            0b00000011 => Mode::Reg,
        }
    }
}

#[derive(Debug)]
enum CmdParsingState {
    First,
    Second,
    Hi,
    Lo,
    Data8,
    Data16,
}

impl Command {
    fn get_mem_registers(item: u8, mode: Mode) -> Option<Memory> {
        match item & 0b00000111 {
            0b00000000 => Some(Memory {
                registers: Some(vec![Register::BX, Register::SI]),
                displacements: None,
            }),
            0b00000001 => Some(Memory {
                registers: Some(vec![Register::BX, Register::DI]),
                displacements: None,
            }),
            0b00000010 => Some(Memory {
                registers: Some(vec![Register::BP, Register::SI]),
                displacements: None,
            }),
            0b00000011 => Some(Memory {
                registers: Some(vec![Register::BP, Register::DI]),
                displacements: None,
            }),
            0b00000100 => Some(Memory {
                registers: Some(vec![Register::SI]),
                displacements: None,
            }),
            0b00000101 => Some(Memory {
                registers: Some(vec![Register::DI]),
                displacements: None,
            }),
            0b00000110 => {
                if mode == Mode::MemNoDisplacement {
                    Some(Memory {
                        registers: None,
                        displacements: None,
                    })
                } else {
                    Some(Memory {
                        registers: Some(vec![Register::BP]),
                        displacements: None,
                    })
                }
            }
            0b00000111 => Some(Memory {
                registers: Some(vec![Register::BX]),
                displacements: None,
            }),
            _ => {
                println!("unsupported byte squence");
                None
            }
        }
    }

    pub fn from(items: &[u8]) -> Vec<Command> {
        let mut res = Vec::new();

        let mut state = CmdParsingState::First;
        let mut cur_cmd = Command::default();
        let mut w = false;
        let mut d = false;
        let mut mode: Mode;
        let mut mem: Memory;
        let mut reset = false;

        for item in items {
            match state {
                First => {
                    if cur_cmd.instruction != Instruction::Nan {
                        res.push(cur_cmd);
                        reset = true;
                    }

                    if reset {
                        cur_cmd = Command::default();
                        mem = Memory::default();
                        reset = false;
                    }
                    cur_cmd.instruction = Instruction::from(*item);
                    match cur_cmd.instruction {
                        Instruction::Mov(m) => match m {
                            MovVariant::AccToMem => {
                                w = (item & 0b00000001) == 0b00000001;
                                state = CmdParsingState::Lo;
                                cur_cmd.source = Storage::Reg(Register::AX);
                            }
                            MovVariant::MemToAcc => {
                                w = (item & 0b00000001) == 0b00000001;
                                state = CmdParsingState::Lo;
                                cur_cmd.destination = Storage::Reg(Register::AX);
                            }
                            MovVariant::ToFromReg => {
                                d = (item & 0b00000010) == 0b00000010;
                                w = (item & 0b00000001) == 0b00000001;
                                state = CmdParsingState::Second;
                            }
                            MovVariant::ImmediateToReg => {
                                w = (item & 0b00001000) == 0b00001000;
                                cur_cmd.destination =
                                    Storage::Reg(Register::from_reg(item & 0b00000111, w));
                                state = CmdParsingState::Data8;
                            }
                            MovVariant::ImmediateToStorage => {
                                w = (item & 0b00000001) == 0b00000001;
                                state = CmdParsingState::Second;
                            }
                        },
                        Instruction::Nan => {
                            println!("Invalid instruction found");
                            return Vec::default();
                        }
                    }
                }
                CmdParsingState::Second => match cur_cmd.instruction {
                    Instruction::Mov(m) => match m {
                        MovVariant::ToFromReg => {
                            mode = Mode::from(item >> 6);
                            let first =
                                Storage::Reg(Register::from_reg((item & 0b00111000) >> 3, w));
                            let second = match mode {
                                Mode::Reg => {
                                    state = CmdParsingState::First;
                                    Storage::Reg(Register::from_reg(item & 0b00000111, w))
                                }
                                Mode::Mem8BitDisplacement
                                | Mode::Mem16BitDisplacement
                                | Mode::MemNoDisplacement => {
                                    match Command::get_mem_registers(*item, mode) {
                                        Some(v) => mem = v,
                                        None => {
                                            state = CmdParsingState::First;
                                            reset = true;
                                        }
                                    }
                                    state = CmdParsingState::Hi;
                                    Storage::Mem(mem)
                                }
                            };
                            if d {
                                cur_cmd.destination = first;
                                cur_cmd.source = second;
                            } else {
                                cur_cmd.source = first;
                                cur_cmd.destination = second;
                            }
                        }
                        MovVariant::ImmediateToStorage => {
                            mode = Mode::from(item >> 6);
                            let storage = match mode {
                                Mode::Reg => {
                                    state = CmdParsingState::Data8;
                                    Storage::Reg(Register::from_reg(item & 0b00000111, w))
                                }
                                Mode::Mem8BitDisplacement
                                | Mode::Mem16BitDisplacement
                                | Mode::MemNoDisplacement => {
                                    match Command::get_mem_registers(*item, mode) {
                                        Some(v) => mem = v,
                                        None => {
                                            state = CmdParsingState::First;
                                            reset = true;
                                        }
                                    }
                                    state = CmdParsingState::Hi;
                                    Storage::Mem(mem)
                                }
                            };
                            cur_cmd.destination = storage;
                        }
                        _ => {
                            println!(
                                "Unable to parse Mov {} in step {:?}",
                                cur_cmd.instruction, state
                            );
                            return Vec::default();
                        }
                    },
                    Instruction::Nan => {
                        println!("Invalid instruction found");
                        return Vec::default();
                    }
                },
                CmdParsingState::Lo => match cur_cmd.instruction {
                    Instruction::Mov(v) => match v {
                        MovVariant::MemToAcc | MovVariant::AccToMem => {
                            mem = Memory {
                                registers: None,
                                displacements: Some((*item, None)),
                            };
                            state = CmdParsingState::Hi;
                        }
                        MovVariant::ToFromReg => {

                            match mode {
                                Mode::MemNoDisplacement => {
                                   if mem.registers.is_none() {
                                        mem.displacements = Some((*item, None));
                                        state = CmdParsingState::Lo;
                                   } else {
                                        state = CmdParsingState::First;
                                    }
                                },
                                    Mode::Mem8BitDisplacement => ,
                                Mode::Mem16BitDisplacement =>,
                                    Mode::Reg => {
                                    println!("You shouldn't come in here");
                                    reset = true;
                                }
                            }

                            state = if w {
                                CmdParsingState::Hi
                            } else {
                                match mode {

                                    }
                                if d {
                                    cur_cmd.
                                }
                                CmdParsingState::First
                            };
                        }
                        MovVariant::ImmediateToStorage => {
                            mem = Memory {
                                registers: None,
                                displacements: Some((*item, None)),
                            };
                            state = if w {
                                CmdParsingState::Hi
                            } else {
                                CmdParsingState::Data8
                            };
                        }
                        _ => {}
                    },
                    Instruction::Nan => {
                        println!("Invalid instruction found");
                        return Vec::default();
                    }
                },
            }
        }

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
