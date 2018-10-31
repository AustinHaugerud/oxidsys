use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddGoldToPartyOp;

const DOC : &str = "Marks the party as carrying the specified amount of gold, which can be pillaged by player if he destroys it. Operation must not be used to give gold to player's party.";

pub const OP_CODE: u32 = 1070;

pub const IDENT: &str = "add_gold_to_party";

impl Operation for AddGoldToPartyOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
