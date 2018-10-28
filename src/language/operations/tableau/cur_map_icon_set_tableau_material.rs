use language::operations::Operation;

pub struct CurMapIconSetTableauMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1983;

pub const IDENT: &str = "cur_map_icon_set_tableau_material";

impl Operation for CurMapIconSetTableauMaterialOp {
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
