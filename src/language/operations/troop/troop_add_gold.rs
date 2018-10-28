use language::operations::Operation;

pub struct TroopAddGoldOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1528;

pub const IDENT: &str = "troop_add_gold";

impl Operation for TroopAddGoldOp {
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
