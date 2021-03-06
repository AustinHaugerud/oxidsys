use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerSetMyTroopOp;

const DOC: &str = "Client operation. Selects a new troop for the player.";

pub const OP_CODE: u32 = 413;

pub const IDENT: &str = "multiplayer_set_my_troop";

impl Operation for MultiplayerSetMyTroopOp {
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
