use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyUpgradeWithXpOp;

const DOC : &str = "Awards specified number of xp points to entire party (split between all stacks) and upgrades all eligible troops. Upgrade direction: (0 = random, 1 = first, 2 = second).";

pub const OP_CODE: u32 = 1673;

pub const IDENT: &str = "party_upgrade_with_xp";

impl Operation for PartyUpgradeWithXpOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<xp_amount>", ""),
                make_param_doc("<upgrade_path>", ""),
            ],
        }
    }
}
