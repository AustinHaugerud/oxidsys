use language::operations::Operation;

pub struct StoreRandomTroopToRaiseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2251;

pub const IDENT: &str = "store_random_troop_to_raise";

impl Operation for StoreRandomTroopToRaiseOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
