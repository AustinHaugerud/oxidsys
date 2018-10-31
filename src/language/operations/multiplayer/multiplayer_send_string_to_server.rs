use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerSendStringToServerOp;

const DOC: &str =
    "Multiplayer client operation. Send a message with a string value to game server.";

pub const OP_CODE: u32 = 393;

pub const IDENT: &str = "multiplayer_send_string_to_server";

impl Operation for MultiplayerSendStringToServerOp {
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
                make_param_doc("<message_type>", ""),
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
