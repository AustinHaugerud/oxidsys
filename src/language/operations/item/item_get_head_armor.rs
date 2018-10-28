use language::operations::Operation;

pub struct ItemGetHeadArmorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2703;

pub const IDENT: &str = "item_get_head_armor";

impl Operation for ItemGetHeadArmorOp {
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
