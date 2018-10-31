use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SendMessageToUrlOp;

const DOC : &str = "Sends an HTTP request. Response from that URL will be returned to \"script_game_receive_url_response\". Parameter <encode_url> is optional and effects are unclear. Supposedly it's equivalent of calling (str_encode_url) on the first parameter which doesn't make sense for me.";

pub const OP_CODE: u32 = 380;

pub const IDENT: &str = "send_message_to_url";

impl Operation for SendMessageToUrlOp {
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
                make_param_doc("<string_id>", ""),
                make_param_doc("<encode_url>", ""),
            ],
        }
    }
}
