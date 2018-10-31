use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetBonePositionOp;

const DOC : &str = "Version 1.161+. Returns current position for agent's bone (examine skeleton in openBrf to learn bone numbers). Pass 1 as optional <local_or_global> parameter to retrieve global bone coordinates.";

pub const OP_CODE: u32 = 2076;

pub const IDENT: &str = "agent_get_bone_position";

impl Operation for AgentGetBonePositionOp {
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
            num_required: 4,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<position_no>", ""),
                make_param_doc("<agent_no>", ""),
                make_param_doc("<bone_no>", ""),
                make_param_doc("<local_or_global>", ""),
                make_param_doc("[<local_or_global>]", ""),
            ],
        }
    }
}
