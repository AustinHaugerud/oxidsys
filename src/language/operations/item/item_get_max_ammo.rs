use language::operations::Operation;

pub struct ItemGetMaxAmmoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2710;

pub const IDENT: &str = "item_get_max_ammo";

impl Operation for ItemGetMaxAmmoOp {
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
