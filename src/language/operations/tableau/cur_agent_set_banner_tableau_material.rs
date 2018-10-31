use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurAgentSetBannerTableauMaterialOp;

const DOC : &str = "Can only be used inside ti_on_agent_spawn trigger in module_mission_templates. Assigns heraldry .";

pub const OP_CODE: u32 = 1986;

pub const IDENT: &str = "cur_agent_set_banner_tableau_material";

impl Operation for CurAgentSetBannerTableauMaterialOp {
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
            param_docs: vec![make_param_doc("<tableau_material_id>", "")],
        }
    }
}
