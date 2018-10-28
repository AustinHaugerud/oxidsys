use language::operations::Operation;

pub struct ContextMenuAddItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 980;

pub const IDENT: &str = "context_menu_add_item";

impl Operation for ContextMenuAddItemOp {
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
