use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetEncounteredPartyOp;

const DOC : &str = "Sets the specified party as encountered by player, but does not run the entire encounter routine. Used in Native during chargen to set up the starting town and then immediately throw the player into street fight without showing him the town menu.";

pub const OP_CODE: u32 = 2205;

pub const IDENT: &str = "set_encountered_party";

impl Operation for SetEncounteredPartyOp {
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
            param_docs: vec![make_param_doc("<party_no>", "")],
        }
    }
}
