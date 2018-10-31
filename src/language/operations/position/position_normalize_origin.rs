use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionNormalizeOriginOp;

const DOC : &str = "What this operation seems to do is calculate the distance between the zero point [0,0,0] and the point with position's coordinates. Can be used to quickly calculate distance to relative positions.";

pub const OP_CODE: u32 = 741;

pub const IDENT: &str = "position_normalize_origin";

impl Operation for PositionNormalizeOriginOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
