use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ModifyVisitorsAtSiteOp;

const DOC: &str = "Declares the scene which visitors will be modified from that moment on.";

pub const OP_CODE: u32 = 1261;

pub const IDENT: &str = "modify_visitors_at_site";

impl Operation for ModifyVisitorsAtSiteOp {
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
            param_docs: vec![make_param_doc("<scene_id>", "")],
        }
    }
}
