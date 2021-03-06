use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerSendIntToServerOp;

const DOC : &str = "Multiplayer client operation. Send a message with a single extra integer value to game server.";

pub const OP_CODE: u32 = 389;

pub const IDENT: &str = "multiplayer_send_int_to_server";

impl Operation for MultiplayerSendIntToServerOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
