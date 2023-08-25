mod assembler;
mod instruction;
mod instruction_format;
mod label;
mod register;

use assembler::Assembler;

// TODO: suporte a registradores com nome
// ex: $t0, $s0, $sp

// TODO: exportar para arquivo binario e
// hexadecimal

fn main() {
    let file_to_read = "assembly.asm";
    let assembler = Assembler::new(file_to_read);
    assembler.assemble();
}
