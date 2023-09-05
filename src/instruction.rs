pub type Address = u16;
pub type Register = usize;

pub struct Opcode(u16);

impl Opcode {
    // OPCODE üzerinde 0x0X00 değerini döner
    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize 
    }

    // OPCODE üzerinde 0x00Y0 değerini döner
    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize 
    }

    // OPCODE üzerinde 0x000N değerini döner 
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) -> as u8 
    }

    // OPCODE üzerinde 0x00NN değerini döner 
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) -> as u8 
    }

    // OPCODE üzerinde 0x0NNN değerini döner
    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
} 

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Opcode {
        Opcode(opcode)
    }
}

pub type Address = u16;
pub type Register = usize;

pub enum Instruction {
    ClearDisplay,                                              //  
    Return,                                                    //
    Jump(Address),                                             //
    Call(Address),                                             //
    SkipIfEqualsByte(Register, u8),                            //
    SkipIfNotEqualsByte(Register, u8),                         //
    SkipIfEqual(Register, Register),                           //
    LoadByte(Register, u8),                                    //
    AddByte(Register, u8),                                     //
    Move(Register, Register),                                  //
    Or(Register, Register),                                    //
    And(Register, Register),                                   //
    Xor(Register, Register),                                   //
    Add(Register, Register),                                   //
    Sub(Register, Register),                                   //
    ShiftRight(Register),                                      //
    ReverseSub(Register, Register),                            //
    ShiftLeft(Register),                                       //
    SkipIfNotEqualsByte(Register, Register),                   //
    LoadI(Address),                                            //
    JumpPlusZero(Address),                                     //
    Random(Register, u8),                                      //
    Draw(Register, Register, u8),                              //
    SkipIfPressed(Register),                                   //
    SkipIfNotPressed(Register),                                //
    LoadDelayTimer(Register),                                  //
    WaitForKeyPress(Register),                                 //
    SetDelayTimer(Register),                                   //
    SetSoundTimer(Register),                                   //
    AddToI(Register),                                          //
    LoadSprite(Register),                                      //
    BCDRepresentation(Register),                               // 
    StoreRegisters(Register),                                  //
    LoadRegisters(Register),                                   //
} 

impl Instruction {
    pub fn new<T: Into<Opcode>>(opcode: T) -> Option<Instruction> {
        let opcode = opcode.into();
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.ooon() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return),
                _ => None,
            },
            0x1000 => Some(Instruction::Jump(opcode.onnn())),
            0x2000 => Some(Instruction::Call(opcode.onnn())),
            0x3000 => Some(Instruction::SkipIfEqualsByte(opcode.oxoo(), opcode.oonn())),
            0x4000 => Some(Instruction::SkipIfNotEqualsByte(
                opcode.oxoo(),
                opcode.oonn(),
            )),
            0x5000 => Some(Instruction::SkipIfEqual(opcode.oxoo(), opcode.ooyo()),
            0x6000 => Some(Instruction::LoadByte(opcode.oxoo(), opcode.oonn())),
            0x7000 => Some(Instruction::AddByte(opcode.oxoo(), opcode.oonn())),
            0x8000 => match opcode.ooon() {
                0x0000 => Some(Instruction::Move(opcode.oxoo(), opcode.ooyo())),
                0x0001 => Some(Instruction::Or(opcode.oxoo(), opcode.oonn())),
                0x0002 => Some(Instruction::And(opcode.oxoo(), opcode.ooyo())),
                0x0003 => Some(Instruction::Xor(opcode.oxoo(), opcode.ooyo())),
                0x0004 => Some(Instruction::Add(opcode.oxoo(), opcode.ooyo())),
                0x0005 => Some(Instruction::Sub(opcode.oxoo(), opcode.ooyo())),
                0x0006 => Some(Instruction::ShiftRight(opcode.oxoo(), opcode.ooyo())),
                0x0007 => Some(Instruction::ReverseSub(opcode.oxoo(), opcode.ooyo())),
                0x000E => Some(Instruction::ShiftLeft(opcode.oxoo())),
                _ => None,
                0x0009 => Some(Instruction::SkipIfNotEqual(opcode.oxoo(), opcode.ooyo())),
                0x000A => Some(Instruction::LoadI(opcode.onnn())),
                0x000B => Some(Instruction::JumpPlusZero(opcode.onnn())),
                0x000C => Some(Instruction::Random(opcode.oxoo(), opcode.oonn())),
                0x000D => Some(Instruction::Draw(
                    opcode.oxoo(),
                    opcode.ooyo(),
                    opcode.ooon(),
                )),
                0xE000 => match opcode.oonn() {
                    0x009E => Some(Instruction::SkipIfPressed(opcode.oxoo())),
                    0x00A1 => Some(Instruction::SkipIfNotPressed(opcode.oxoo())),
                    _ => None,
                },
                0xF000 => match opcode.oonn() {
                    0x0007 => Some(Instruction::LoadDelayTime(opcode.oxoo())),
                    0x000A => Some(Instruction::WaitForKeyPress(opcode.oxoo())),
                    0x0015 => Some(Instruction::SetDelayTimer(opcode.oxoo())),
                    0x0018 => Some(Instruction::SetSoundTimer(opcode.oxoo())),
                    0x001E => Some(Instruction::AddToI(opcode.oxoo())),
                    0x0029 => Some(Instruction::LoadSprite(opcode.oxoo())),
                    0x0033 => Some(Instruction::BCDRepresentation(opcode.oxoo())),
                    0x0055 => Some(Instruction::StoreRegisters(opcode.oxoo())),
                    0x0065 => Some(Instruction::LoadRegisters(opcode.oxoo())),
                    _ => None, 
                },
                _ => None,
            }
            )
        }
    }
}
