use language::operations::Operation;
pub mod close_item_details;
pub mod create_button_overlay;
pub mod create_check_box_overlay;
pub mod create_combo_button_overlay;
pub mod create_combo_label_overlay;
pub mod create_game_button_overlay;
pub mod create_image_button_overlay;
pub mod create_image_button_overlay_with_tableau_material;
pub mod create_listbox_overlay;
pub mod create_mesh_overlay;
pub mod create_mesh_overlay_with_item_id;
pub mod create_mesh_overlay_with_tableau_material;
pub mod create_number_box_overlay;
pub mod create_progress_overlay;
pub mod create_simple_text_box_overlay;
pub mod create_slider_overlay;
pub mod create_text_box_overlay;
pub mod create_text_overlay;
pub mod is_presentation_active;
pub mod overlay_add_item;
pub mod overlay_animate_to_alpha;
pub mod overlay_animate_to_color;
pub mod overlay_animate_to_highlight_alpha;
pub mod overlay_animate_to_highlight_color;
pub mod overlay_animate_to_position;
pub mod overlay_get_position;
pub mod overlay_obtain_focus;
pub mod overlay_set_additional_render_height;
pub mod overlay_set_alpha;
pub mod overlay_set_area_size;
pub mod overlay_set_boundaries;
pub mod overlay_set_color;
pub mod overlay_set_container_overlay;
pub mod overlay_set_display;
pub mod overlay_set_hilight_alpha;
pub mod overlay_set_hilight_color;
pub mod overlay_set_material;
pub mod overlay_set_mesh_rotation;
pub mod overlay_set_position;
pub mod overlay_set_size;
pub mod overlay_set_text;
pub mod overlay_set_tooltip;
pub mod overlay_set_val;
pub mod presentation_set_duration;
pub mod set_container_overlay;
pub mod show_item_details;
pub mod show_item_details_with_modifier;
pub mod show_troop_details;
pub mod start_background_presentation;
pub mod start_presentation;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(close_item_details::CloseItemDetailsOp {}));
    result.push(Box::new(create_button_overlay::CreateButtonOverlayOp {}));
    result.push(Box::new(
        create_check_box_overlay::CreateCheckBoxOverlayOp {},
    ));
    result.push(Box::new(
        create_combo_button_overlay::CreateComboButtonOverlayOp {},
    ));
    result.push(Box::new(
        create_combo_label_overlay::CreateComboLabelOverlayOp {},
    ));
    result.push(Box::new(
        create_game_button_overlay::CreateGameButtonOverlayOp {},
    ));
    result.push(Box::new(
        create_image_button_overlay::CreateImageButtonOverlayOp {},
    ));
    result.push(Box::new(create_image_button_overlay_with_tableau_material::CreateImageButtonOverlayWithTableauMaterialOp {}));
    result.push(Box::new(create_listbox_overlay::CreateListboxOverlayOp {}));
    result.push(Box::new(create_mesh_overlay::CreateMeshOverlayOp {}));
    result.push(Box::new(
        create_mesh_overlay_with_item_id::CreateMeshOverlayWithItemIdOp {},
    ));
    result.push(Box::new(
        create_mesh_overlay_with_tableau_material::CreateMeshOverlayWithTableauMaterialOp {},
    ));
    result.push(Box::new(
        create_number_box_overlay::CreateNumberBoxOverlayOp {},
    ));
    result.push(Box::new(
        create_progress_overlay::CreateProgressOverlayOp {},
    ));
    result.push(Box::new(
        create_simple_text_box_overlay::CreateSimpleTextBoxOverlayOp {},
    ));
    result.push(Box::new(create_slider_overlay::CreateSliderOverlayOp {}));
    result.push(Box::new(create_text_box_overlay::CreateTextBoxOverlayOp {}));
    result.push(Box::new(create_text_overlay::CreateTextOverlayOp {}));
    result.push(Box::new(is_presentation_active::IsPresentationActiveOp {}));
    result.push(Box::new(overlay_add_item::OverlayAddItemOp {}));
    result.push(Box::new(
        overlay_animate_to_alpha::OverlayAnimateToAlphaOp {},
    ));
    result.push(Box::new(
        overlay_animate_to_color::OverlayAnimateToColorOp {},
    ));
    result.push(Box::new(
        overlay_animate_to_highlight_alpha::OverlayAnimateToHighlightAlphaOp {},
    ));
    result.push(Box::new(
        overlay_animate_to_highlight_color::OverlayAnimateToHighlightColorOp {},
    ));
    result.push(Box::new(
        overlay_animate_to_position::OverlayAnimateToPositionOp {},
    ));
    result.push(Box::new(overlay_get_position::OverlayGetPositionOp {}));
    result.push(Box::new(overlay_obtain_focus::OverlayObtainFocusOp {}));
    result.push(Box::new(
        overlay_set_additional_render_height::OverlaySetAdditionalRenderHeightOp {},
    ));
    result.push(Box::new(overlay_set_alpha::OverlaySetAlphaOp {}));
    result.push(Box::new(overlay_set_area_size::OverlaySetAreaSizeOp {}));
    result.push(Box::new(overlay_set_boundaries::OverlaySetBoundariesOp {}));
    result.push(Box::new(overlay_set_color::OverlaySetColorOp {}));
    result.push(Box::new(
        overlay_set_container_overlay::OverlaySetContainerOverlayOp {},
    ));
    result.push(Box::new(overlay_set_display::OverlaySetDisplayOp {}));
    result.push(Box::new(
        overlay_set_hilight_alpha::OverlaySetHilightAlphaOp {},
    ));
    result.push(Box::new(
        overlay_set_hilight_color::OverlaySetHilightColorOp {},
    ));
    result.push(Box::new(overlay_set_material::OverlaySetMaterialOp {}));
    result.push(Box::new(
        overlay_set_mesh_rotation::OverlaySetMeshRotationOp {},
    ));
    result.push(Box::new(overlay_set_position::OverlaySetPositionOp {}));
    result.push(Box::new(overlay_set_size::OverlaySetSizeOp {}));
    result.push(Box::new(overlay_set_text::OverlaySetTextOp {}));
    result.push(Box::new(overlay_set_tooltip::OverlaySetTooltipOp {}));
    result.push(Box::new(overlay_set_val::OverlaySetValOp {}));
    result.push(Box::new(
        presentation_set_duration::PresentationSetDurationOp {},
    ));
    result.push(Box::new(set_container_overlay::SetContainerOverlayOp {}));
    result.push(Box::new(show_item_details::ShowItemDetailsOp {}));
    result.push(Box::new(
        show_item_details_with_modifier::ShowItemDetailsWithModifierOp {},
    ));
    result.push(Box::new(show_troop_details::ShowTroopDetailsOp {}));
    result.push(Box::new(
        start_background_presentation::StartBackgroundPresentationOp {},
    ));
    result.push(Box::new(start_presentation::StartPresentationOp {}));
    result
}
