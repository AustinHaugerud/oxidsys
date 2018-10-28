use language::operations::Operation;

pub struct ItemGetLegArmorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2705;

pub const IDENT: &str = "item_get_leg_armor";

impl Operation for ItemGetLegArmorOp {
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
