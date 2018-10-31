use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionCopyOriginOp;

const DOC : &str = "Copies coordinates from source position to target position, without changing rotation or scale.";

pub const OP_CODE: u32 = 719;

pub const IDENT: &str = "position_copy_origin";

impl Operation for PositionCopyOriginOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<position_target>", ""),
                make_param_doc("<position_source>", ""),
            ],
        }
    }
}
