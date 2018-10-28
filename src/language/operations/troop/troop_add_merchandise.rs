use language::operations::Operation;

pub struct TroopAddMerchandiseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1512;

pub const IDENT: &str = "troop_add_merchandise";

impl Operation for TroopAddMerchandiseOp {
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
