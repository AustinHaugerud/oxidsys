use language::operations::Operation;

pub struct CurItemSetTableauMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1981;

pub const IDENT: &str = "cur_item_set_tableau_material";

impl Operation for CurItemSetTableauMaterialOp {
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
