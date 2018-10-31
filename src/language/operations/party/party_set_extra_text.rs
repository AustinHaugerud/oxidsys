use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetExtraTextOp;

const DOC : &str = "Allows to put extra text in party details popup. Used in Native to set status for villages or towns (being raided, razed, under siege...).";

pub const OP_CODE: u32 = 1605;

pub const IDENT: &str = "party_set_extra_text";

impl Operation for PartySetExtraTextOp {
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
                make_param_doc("<string>", ""),
            ],
        }
    }
}
