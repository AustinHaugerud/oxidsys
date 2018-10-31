use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetBannerIconOp;

const DOC: &str =
    "Sets what map icon will be used as the party banner. Use 0 to remove banner from a party.";

pub const OP_CODE: u32 = 1677;

pub const IDENT: &str = "party_set_banner_icon";

impl Operation for PartySetBannerIconOp {
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
                make_param_doc("<map_icon_id>", ""),
            ],
        }
    }
}
