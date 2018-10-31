use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreAgentNameOp;

const DOC: &str = "Stores agent name in referenced string register.";

pub const OP_CODE: u32 = 2332;

pub const IDENT: &str = "str_store_agent_name";

impl Operation for StrStoreAgentNameOp {
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
                make_param_doc("<string_register>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
