use language::operations::Operation;

pub struct TroopSetPluralNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1502;

pub const IDENT: &str = "troop_set_plural_name";

impl Operation for TroopSetPluralNameOp {
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
