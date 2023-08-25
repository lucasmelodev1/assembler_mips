pub const REGISTERS: [Register; 32] = [
    Register {
        number: 0,
        name: "zero",
    },
    Register {
        number: 1,
        name: "at",
    },
    Register {
        number: 2,
        name: "v0",
    },
    Register {
        number: 3,
        name: "v1",
    },
    Register {
        number: 4,
        name: "a0",
    },
    Register {
        number: 5,
        name: "a1",
    },
    Register {
        number: 6,
        name: "a2",
    },
    Register {
        number: 7,
        name: "a3",
    },
    Register {
        number: 8,
        name: "t0",
    },
    Register {
        number: 9,
        name: "t1",
    },
    Register {
        number: 10,
        name: "t2",
    },
    Register {
        number: 11,
        name: "t3",
    },
    Register {
        number: 12,
        name: "t4",
    },
    Register {
        number: 13,
        name: "t5",
    },
    Register {
        number: 14,
        name: "t6",
    },
    Register {
        number: 15,
        name: "t7",
    },
    Register {
        number: 16,
        name: "s0",
    },
    Register {
        number: 17,
        name: "s1",
    },
    Register {
        number: 18,
        name: "s2",
    },
    Register {
        number: 19,
        name: "s3",
    },
    Register {
        number: 20,
        name: "s4",
    },
    Register {
        number: 21,
        name: "s5",
    },
    Register {
        number: 22,
        name: "s6",
    },
    Register {
        number: 23,
        name: "s7",
    },
    Register {
        number: 24,
        name: "t8",
    },
    Register {
        number: 25,
        name: "t9",
    },
    Register {
        number: 26,
        name: "k0",
    },
    Register {
        number: 27,
        name: "k1",
    },
    Register {
        number: 28,
        name: "gp",
    },
    Register {
        number: 29,
        name: "sp",
    },
    Register {
        number: 30,
        name: "fp",
    },
    Register {
        number: 31,
        name: "ra",
    },
];

pub struct Register {
    pub number: u8,
    pub name: &'static str,
}

impl Register {
    fn named_to_number(name: &str) -> u8 {
        REGISTERS
            .iter()
            .find(|register| register.name == name)
            .expect("Registrador nao encontrado")
            .number
    }

    fn filter_string(unfiltered_string: &str) -> String {
        let mut register = unfiltered_string.chars();
        register.next();
        if register.clone().last() == Some(',') {
            register.next_back();
        }
        register.as_str().to_string()
    }

    pub fn get_register(unfiltered_string: &str) -> u8 {
        let register_string = Register::filter_string(unfiltered_string);
        match register_string.parse::<u8>() {
            Ok(value) => return value,
            Err(_) => Register::named_to_number(register_string.as_ref()),
        }
    }
}
