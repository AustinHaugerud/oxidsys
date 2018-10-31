use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveTroopFromSiteOp;

const DOC: &str = "Removes the troop from the specified scene. No longer used in Native.";

pub const OP_CODE: u32 = 1251;

pub const IDENT: &str = "remove_troop_from_site";

impl Operation for RemoveTroopFromSiteOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<scene_id>", ""),
            ],
        }
    }
}
