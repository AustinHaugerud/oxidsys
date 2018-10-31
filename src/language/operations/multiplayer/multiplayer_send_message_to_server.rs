use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerSendMessageToServerOp;

const DOC : &str = "Multiplayer client operation. Send a simple message (only message code, no data) to game server.";

pub const OP_CODE: u32 = 388;

pub const IDENT: &str = "multiplayer_send_message_to_server";

impl Operation for MultiplayerSendMessageToServerOp {
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
            param_docs: vec![make_param_doc("<message_type>", "")],
        }
    }
}
