use language::operations::Operation;

pub struct TroopHasItemEquippedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 151;

pub const IDENT: &str = "troop_has_item_equipped";

impl Operation for TroopHasItemEquippedOp {
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
