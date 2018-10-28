use language::operations::Operation;

pub struct ItemGetSwingDamageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2720;

pub const IDENT: &str = "item_get_swing_damage";

impl Operation for ItemGetSwingDamageOp {
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
