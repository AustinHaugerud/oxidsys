use language::operations::Operation;

pub struct TroopAddMerchandiseWithFactionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1513;

pub const IDENT: &str = "troop_add_merchandise_with_faction";

impl Operation for TroopAddMerchandiseWithFactionOp {
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
