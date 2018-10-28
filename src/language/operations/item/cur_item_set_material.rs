use language::operations::Operation;

pub struct CurItemSetMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1978;

pub const IDENT: &str = "cur_item_set_material";

impl Operation for CurItemSetMaterialOp {
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
