use crate::instruction_format::InstructionFormat;

pub const INSTRUCTIONS: [Instruction; 31] = [
    // FORMAT R
    Instruction {
        name: "sll",
        format: InstructionFormat::R {
            op_code: 0,
            function: 0,
        },
    },
    Instruction {
        name: "srl",
        format: InstructionFormat::R {
            op_code: 0,
            function: 2,
        },
    },
    Instruction {
        name: "jr",
        format: InstructionFormat::R {
            op_code: 0,
            function: 8,
        },
    },
    Instruction {
        name: "mfhi",
        format: InstructionFormat::R {
            op_code: 0,
            function: 16,
        },
    },
    Instruction {
        name: "mflo",
        format: InstructionFormat::R {
            op_code: 0,
            function: 18,
        },
    },
    Instruction {
        name: "mult",
        format: InstructionFormat::R {
            op_code: 0,
            function: 24,
        },
    },
    Instruction {
        name: "multu",
        format: InstructionFormat::R {
            op_code: 0,
            function: 25,
        },
    },
    Instruction {
        name: "div",
        format: InstructionFormat::R {
            op_code: 0,
            function: 26,
        },
    },
    Instruction {
        name: "divu",
        format: InstructionFormat::R {
            op_code: 0,
            function: 27,
        },
    },
    Instruction {
        name: "add",
        format: InstructionFormat::R {
            op_code: 0,
            function: 32,
        },
    },
    Instruction {
        name: "addu",
        format: InstructionFormat::R {
            op_code: 0,
            function: 33,
        },
    },
    Instruction {
        name: "sub",
        format: InstructionFormat::R {
            op_code: 0,
            function: 34,
        },
    },
    Instruction {
        name: "subu",
        format: InstructionFormat::R {
            op_code: 0,
            function: 35,
        },
    },
    Instruction {
        name: "and",
        format: InstructionFormat::R {
            op_code: 0,
            function: 36,
        },
    },
    Instruction {
        name: "or",
        format: InstructionFormat::R {
            op_code: 0,
            function: 37,
        },
    },
    Instruction {
        name: "slt",
        format: InstructionFormat::R {
            op_code: 0,
            function: 42,
        },
    },
    Instruction {
        name: "sltu",
        format: InstructionFormat::R {
            op_code: 0,
            function: 43,
        },
    },
    Instruction {
        name: "mul",
        format: InstructionFormat::R {
            op_code: 28,
            function: 2,
        },
    },
    // FORMAT I
    Instruction {
        name: "beq",
        format: InstructionFormat::I { op_code: 4 },
    },
    Instruction {
        name: "bne",
        format: InstructionFormat::I { op_code: 5 },
    },
    Instruction {
        name: "addi",
        format: InstructionFormat::I { op_code: 8 },
    },
    Instruction {
        name: "addiu",
        format: InstructionFormat::I { op_code: 9 },
    },
    Instruction {
        name: "slti",
        format: InstructionFormat::I { op_code: 10 },
    },
    Instruction {
        name: "sltiu",
        format: InstructionFormat::I { op_code: 10 },
    },
    Instruction {
        name: "andi",
        format: InstructionFormat::I { op_code: 12 },
    },
    Instruction {
        name: "ori",
        format: InstructionFormat::I { op_code: 13 },
    },
    Instruction {
        name: "lui",
        format: InstructionFormat::I { op_code: 15 },
    },
    Instruction {
        name: "lw",
        format: InstructionFormat::I { op_code: 35 },
    },
    Instruction {
        name: "sw",
        format: InstructionFormat::I { op_code: 43 },
    },
    // FORMAT J
    Instruction {
        name: "j",
        format: InstructionFormat::J { op_code: 2 },
    },
    Instruction {
        name: "jal",
        format: InstructionFormat::J { op_code: 3 },
    },
];

pub struct Instruction {
    pub name: &'static str,
    pub format: InstructionFormat,
}

impl Instruction {
    pub fn get_instruction(name: &str) -> Option<&Instruction> {
        INSTRUCTIONS.iter().find(|predicate| predicate.name == name)
    }
}
