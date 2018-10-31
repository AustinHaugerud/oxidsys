use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenExchangeMembersOp;

const DOC : &str = "Opens the Exchange Members With Party interface, using the specified party_id. If called during an encounter, party_id is optional and defaults to the encountered party. Second parameter determines whether the party leader is exchangeable (useful when managing the castle garrison).";

pub const OP_CODE: u32 = 2043;

pub const IDENT: &str = "change_screen_exchange_members";

impl Operation for ChangeScreenExchangeMembersOp {
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
            num_required: 0,
            num_optional: 2,
            param_docs: vec![
                make_param_doc("[exchange_leader]", ""),
                make_param_doc("[party_id]", ""),
            ],
        }
    }
}
