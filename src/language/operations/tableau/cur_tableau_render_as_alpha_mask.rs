use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauRenderAsAlphaMaskOp;

const DOC: &str = "Tells the engine to treat the tableau as an alpha (transparency) mask.";

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

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
