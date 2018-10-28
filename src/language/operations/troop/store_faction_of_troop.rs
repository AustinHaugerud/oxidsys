use language::operations::Operation;

pub struct StoreFactionOfTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2173;

pub const IDENT: &str = "store_faction_of_troop";

impl Operation for StoreFactionOfTroopOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
