use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionCopyRotationOp;

const DOC : &str = "Copies rotation from source position to target position, without changing coordinates or scale.";

pub const OP_CODE: u32 = 718;

pub const IDENT: &str = "position_copy_rotation";

impl Operation for PositionCopyRotationOp {
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
