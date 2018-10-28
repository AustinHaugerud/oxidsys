use language::operations::Operation;

pub struct CurTableauRenderAsAlphaMaskOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1984;

pub const IDENT: &str = "cur_tableau_render_as_alpha_mask";

impl Operation for CurTableauRenderAsAlphaMaskOp {
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
