pub enum InstructionFormat {
    R { op_code: u8, function: u8 },
    I { op_code: u8 },
    J { op_code: u8 },
}
