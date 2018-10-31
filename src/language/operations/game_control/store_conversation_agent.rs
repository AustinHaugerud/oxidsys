use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreConversationAgentOp;

const DOC: &str = "Stores identifier of agent who is currently speaking.";

pub const OP_CODE: u32 = 2199;

pub const IDENT: &str = "store_conversation_agent";

impl Operation for StoreConversationAgentOp {
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
