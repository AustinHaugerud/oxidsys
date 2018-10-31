use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DistributePartyAmongPartyGroupOp;

const DOC : &str = "Distributes troops from first party among all parties attached to the second party. Commonly used to divide prisoners and resqued troops among NPC parties.";

pub const OP_CODE: u32 = 1698;

pub const IDENT: &str = "distribute_party_among_party_group";

impl Operation for DistributePartyAmongPartyGroupOp {
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
                make_param_doc("<party_to_be_distributed>", ""),
                make_param_doc("<group_root_party>", ""),
            ],
        }
    }
}
