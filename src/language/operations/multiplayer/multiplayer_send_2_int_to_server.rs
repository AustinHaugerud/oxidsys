use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerSend2IntToServerOp;

const DOC: &str = "Same as (multiplayer_send_int_to_server), but two integer values are sent.";

pub const OP_CODE: u32 = 390;

pub const IDENT: &str = "multiplayer_send_2_int_to_server";

impl Operation for MultiplayerSend2IntToServerOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<message_type>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
