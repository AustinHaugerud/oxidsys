use language::operations::Operation;

pub struct ItemGetBodyArmorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2704;

pub const IDENT: &str = "item_get_body_armor";

impl Operation for ItemGetBodyArmorOp {
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
