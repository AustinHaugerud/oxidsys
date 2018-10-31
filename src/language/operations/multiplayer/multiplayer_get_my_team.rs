use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerGetMyTeamOp;

const DOC: &str = "Client operation. Retrieves player's currently selected team.";

pub const OP_CODE: u32 = 411;

pub const IDENT: &str = "multiplayer_get_my_team";

impl Operation for MultiplayerGetMyTeamOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
