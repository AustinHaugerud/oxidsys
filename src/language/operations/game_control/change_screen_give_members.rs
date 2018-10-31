use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenGiveMembersOp;

const DOC : &str = "Opens the Give Troops to Another Party interface. Party_id parameter is optional during an encounter and will use encountered party as default value.";

pub const OP_CODE: u32 = 2056;

pub const IDENT: &str = "change_screen_give_members";

impl Operation for ChangeScreenGiveMembersOp {
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
            num_optional: 1,
            param_docs: vec![make_param_doc("[party_id]", "")],
        }
    }
}
