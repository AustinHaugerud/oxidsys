use std::collections::HashMap;

const NUM_N_REGISTERS : u64 = 66;
const NUM_S_REGISTERS : u64 = 68;
const NUM_P_REGISTERS : u64 = 64;

const TAG_REGISTER : u64 = 1;
const OP_NUM_VALUE_BITS : u64 = 24 + 32;
const OPMASK_REGISTER : u64 = TAG_REGISTER << OP_NUM_VALUE_BITS;

fn register(regnum : u64) -> u64 {
    OPMASK_REGISTER | regnum
}

// Numeric
lazy_static! {

    pub static ref NREGISTERS : HashMap<String, u64> = {
        let mut m = HashMap::new();

        for i in 0..NUM_N_REGISTERS {
            let reg = register(i);
            m.insert(format!("reg{}", i), reg);
        }

        m
    };

    pub static ref SREGISTERS : HashMap<String, u64> = {
        let mut m = HashMap::new();

        for i in 0..NUM_S_REGISTERS {
            m.insert(format!("s{}", i), i);
        }

        m
    };

    pub static ref PREGISTERS : HashMap<String, u64> = {
        let mut m = HashMap::new();

        for i in 0..NUM_P_REGISTERS {
            m.insert(format!("p{}", i), i);
        }

        m
    };
}
