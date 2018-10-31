use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionMoveZOp;

const DOC : &str = "Moves position along Z axis. Movement distance is in cms. Optional parameter determines whether the position is moved along the local (value=0) or global (value=1) Z axis (i.e. whether the position will be moved to it's above/below, or to the global above/below - these directions will be different if the position is tilted).";

pub const OP_CODE: u32 = 722;

pub const IDENT: &str = "position_move_z";

impl Operation for PositionMoveZOp {
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
