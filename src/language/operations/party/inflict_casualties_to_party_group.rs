use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct InflictCasualtiesToPartyGroupOp;

const DOC : &str = "Delivers auto-calculated damage to the party (and all other parties attached to it). Killed troops are moved to another party to keep track of.";

pub const OP_CODE: u32 = 1697;

pub const IDENT: &str = "inflict_casualties_to_party_group";

impl Operation for InflictCasualtiesToPartyGroupOp {
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
                make_param_doc("<parent_party_id>", ""),
                make_param_doc("<damage_amount>", ""),
                make_param_doc("<party_id_to_add_causalties_to>", ""),
            ],
        }
    }
}
