use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPostfxOp;

const DOC : &str = "This operation is not documented nor any examples of it's use could be found. Parameters are unknown.";

pub const OP_CODE: u32 = 2386;

pub const IDENT: &str = "set_postfx";

impl Operation for SetPostfxOp {
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
