use language::operations::Operation;

pub struct ItemGetSwingDamageTypeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2721;

pub const IDENT: &str = "item_get_swing_damage_type";

impl Operation for ItemGetSwingDamageTypeOp {
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
