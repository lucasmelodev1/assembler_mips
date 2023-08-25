use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::{
    instruction::Instruction, instruction_format::InstructionFormat, label::Label,
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
                    let has_rt = words[3].as_bytes()[0] == b'$';

                    if has_rt {
                        let rt = Register::get_register(words[3]);
                        binary.push_str(&format!(
                            "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                            op_code, rs, rt, rd, 0, function
                        ));
                    } else {
                        let shamt = words[3].parse::<u8>().expect("Shamt de 8 bits invalido");
                        binary.push_str(&format!(
                            "{:06b}{:05b}{:05b}{:05b}{:05b}{:06b}",
                            op_code, rs, 0, rd, shamt, function
                        ));
                    }
                }
                InstructionFormat::I { op_code } => {
                    let rs = Register::get_register(words[1]);
                    let rt = Register::get_register(words[2]);

                    let constant = words[3];

                    if Label::is_label_reference(&constant) {
                        let relative_line = Label::reference_to_relative_line(
                            &self.labels,
                            &constant,
                            current_line_number,
                        );
                        binary.push_str(&format!(
                            "{:06b}{:05b}{:05b}{:016b}",
                            op_code, rs, rt, relative_line
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
                }
                InstructionFormat::J { op_code } => {
                    let constant = words[1];

                    if Label::is_label_reference(&constant) {
                        let relative_line = Label::reference_to_relative_line(
                            &self.labels,
                            &constant,
                            current_line_number,
                        );

                        // Conversion from a 32 bit integer to 26 bit integer
                        let masked_number = relative_line & ((1 << 26) - 1);

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
        let n: u32 =
            u32::from_str_radix(val, 2).expect("Erro ao converter binario para hexadecimal");
        format!("{:01$x}", n, len * 2)
    }

    pub fn new(file_to_read: &str) -> Assembler {
        Assembler {
            file_to_read,
            labels: Label::find_labels(file_to_read),
        }
    }
}
