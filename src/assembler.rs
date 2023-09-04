use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::{
    instruction::{self, Instruction},
    instruction_format::InstructionFormat,
    label::Label,
    register::Register,
};

pub struct Assembler<'a> {
    file_to_read: &'a str,
    labels: Vec<Label>,
}

impl<'a> Assembler<'a> {
    /// Assemble a given file and writes the result
    /// binary in a new file
    pub fn assemble(&self) {
        let file_in = File::open(self.file_to_read).expect("Erro ao ler arquivo de entrada");
        let reader = BufReader::new(file_in);
        let mut lines_written = 0;

        let mut file_out = File::create("out.bin").expect("Erro ao criar arquivo de saída");
        file_out
            .write(b"v2.0 raw\n")
            .expect("Erro ao escrever no arquivo");

        for (index, line) in reader.lines().enumerate() {
            // Lines start at 1
            let current_line_number = index + 1;
            let line = line.expect("Erro ao ler linha");

            // each line should respect the following pattern:
            // -> words[0] = instruction (such as "add" or "jal")
            // -> word[1] = rd (in R instructions), rs (in I instructions),
            // or constant (in J instructions)
            // -> word[2] = rs (in R instructions), or rt (in I instructions)
            // -> word[3] = rt or shamt (in R instructions), or constant (in I instructions)
            let mut words: Vec<&str> = line.split_whitespace().collect();

            if words.len() < 1 {
                continue;
            }

            // Does not consider the label when parsing the line
            // because labels were already parsed
            if Label::is_label(words[0]) {
                words.remove(0);
            }

            let instruction =
                Instruction::get_instruction(words[0]).expect("Instrucao nao encontrada");
            let mut binary = String::new();

            match instruction.format {
                InstructionFormat::R { op_code, function } => {
                    let rd = Register::get_register(words[1]);
                    let rs = Register::get_register(words[2]);
                    let mut push_binary = || {
                        if ["mult", "multu", "div", "divu"].contains(&instruction.name) {
                            let rs = Register::get_register(words[1]);
                            let rt = Register::get_register(words[2]);
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                                op_code, rs, rt, 0, 0, function
                            ));
                            return;
                        } else if ["jr", "mfhi", "mflo"].contains(&instruction.name) {
                            if instruction.name == "jr" {
                                let rs = Register::get_register(words[1]);
                                binary.push_str(&format!(
                                    "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                                    op_code, rs, 0, 0, 0, function
                                ));
                            }
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                                op_code, 0, 0, rd, 0, function
                            ));
                            return;
                        }
                        let has_shamt = words[3].as_bytes()[0] != b'$';

                        if has_shamt {
                            let rt = rs;
                            let shamt = words[3].parse::<u8>().expect("Shamt de 8 bits invalido");
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                                op_code, 0, rt, rd, shamt, function
                            ));
                        } else {
                            let rt = Register::get_register(words[3]);
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                                op_code, rs, rt, rd, 0, function
                            ));
                        }
                    };
                    push_binary();
                }
                InstructionFormat::I { op_code } => {
                    let mut push_binary = || {
                        if ["lw", "sw"].contains(&instruction.name) {
                            let rt = Register::get_register(words[1]);
                            let offset = &words[2];
                            let rs = &words[3][1..words[3].len() - 1];

                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:016b}",
                                op_code,
                                Register::get_register(rs),
                                rt,
                                offset.parse::<i16>().unwrap()
                            ));
                            return;
                        }
                        let rs = Register::get_register(words[1]);
                        let rt = Register::get_register(words[2]);

                        if &instruction.name == &"lui" {
                            let rt = Register::get_register(words[1]);
                            let constant = words[2];

                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:016b}",
                                op_code,
                                0,
                                rt,
                                constant.parse::<i16>().unwrap()
                            ));
                            return;
                        }

                        let constant = words[3];

                        if Label::is_label_reference(&constant) {
                            let relative_line = Label::reference_to_relative_line(
                                &self.labels,
                                &constant,
                                current_line_number,
                            );
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:016b}",
                                op_code, rs, rt, relative_line as i16
                            ));
                        } else {
                            binary.push_str(&format!(
                                "{:06b}{:05b}{:05b}{:016b}",
                                op_code,
                                rs,
                                rt,
                                constant
                                    .parse::<i16>()
                                    .expect("Constante de 16 bits invalida")
                            ));
                        }
                    };
                    push_binary();
                }
                InstructionFormat::J { op_code } => {
                    let constant = words[1];

                    if Label::is_label_reference(&constant) {
                        let line = Label::find_label_line_address(&self.labels, &constant);

                        // Conversion from a 32 bit integer to 26 bit integer
                        let masked_number = line & 0x03FFFFFF;

                        binary.push_str(&format!("{:06b}{:026b}", op_code, masked_number));
                    } else {
                        let masked_number = constant
                            .parse::<i32>()
                            .expect("Constante de 32 bits invalida")
                            & ((1 << 26) - 1);
                        binary.push_str(&format!("{:06b}{:026b}", op_code, masked_number));
                    }
                }
            };

            file_out
                .write(Assembler::to_hex(&binary, 0).as_bytes())
                .expect("Erro ao escrever no arquivo de saida");
            lines_written += 1;

            if lines_written % 4 == 0 {
                file_out
                    .write(b"\n")
                    .expect("Erro ao escrever no arquivo de saida");
                continue;
            }
            file_out
                .write(b" ")
                .expect("Erro ao escrever no arquivo de saida");
        }
    }

    fn to_hex(val: &str, len: usize) -> String {
        // let n: i32 =
        //     i32::from_str_radix(val, 2).expect("Erro ao converter binario para hexadecimal");
        // format!("{:01$x}", n, len * 2)
        // Parse the binary string as a u32 integer
        println!("{}", val);
        let decimal_value = u32::from_str_radix(val, 2);

        // Check if the parsing was successful
        match decimal_value {
            Ok(value) => {
                // Format the u32 integer as a hexadecimal string with a "0x" prefix
                format!("0x{:08x}", value)
            }
            Err(_) => {
                // Handle parsing errors (invalid binary string)
                "Invalid binary string".to_string()
            }
        }
    }

    pub fn new(file_to_read: &str) -> Assembler {
        Assembler {
            file_to_read,
            labels: Label::find_labels(file_to_read),
        }
    }
}
