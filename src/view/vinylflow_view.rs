use gui::core::gui_application::GUIApplication;

pub struct VinylFlowView;

impl VinylFlowView {
	pub fn new() -> VinylFlowView {
		VinylFlowView
	}
}

impl GUIApplication for VinylFlowView {
	fn title(&self) -> String { "VinylFlow".to_string() }
	
	fn width(&self) -> u32 { 640 }
	
	fn height(&self) -> u32 { 480 }
}
