use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionMoveXOp;

const DOC : &str = "Moves position along X axis. Movement distance is in cms. Optional parameter determines whether the position is moved along the local (value=0) or global (value=1) X axis (i.e. whether the position will be moved to it's right/left, or to the global east/west).";

pub const OP_CODE: u32 = 720;

pub const IDENT: &str = "position_move_x";

impl Operation for PositionMoveXOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<position>", ""),
                make_param_doc("<movement>", ""),
                make_param_doc("[value]", ""),
            ],
        }
    }
}
