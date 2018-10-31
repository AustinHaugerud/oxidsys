use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateComboButtonOverlayOp;

const DOC : &str = "Creates a combo button overlay. For example see \"Screen Resolution\" dropdown in Settings menu. Individual items can be added with (overlay_add_item) and currently selected item can be set with (overlay_set_val). Returns combo block's overlay_id.";

pub const OP_CODE: u32 = 916;

pub const IDENT: &str = "create_combo_button_overlay";

impl Operation for CreateComboButtonOverlayOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
