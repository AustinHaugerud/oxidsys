use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartMapConversationOp;

const DOC : &str = "Starts a conversation with the selected troop. Can be called directly from global map or game menus. Troop DNA parameter allows you to randomize non-hero troop appearances.";

pub const OP_CODE: u32 = 1025;

pub const IDENT: &str = "start_map_conversation";

impl Operation for StartMapConversationOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("[troop_dna]", ""),
            ],
        }
    }
}
