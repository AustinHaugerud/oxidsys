use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetFactionOp;

const DOC: &str = "Sets party faction allegiance. Party color is changed appropriately.";

pub const OP_CODE: u32 = 1620;

pub const IDENT: &str = "party_set_faction";

impl Operation for PartySetFactionOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<faction_id>", ""),
            ],
        }
    }
}
