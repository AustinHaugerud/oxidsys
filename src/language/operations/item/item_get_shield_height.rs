use language::operations::Operation;

pub struct ItemGetShieldHeightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2712;

pub const IDENT: &str = "item_get_shield_height";

impl Operation for ItemGetShieldHeightOp {
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
