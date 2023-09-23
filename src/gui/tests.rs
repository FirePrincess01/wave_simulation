
use super::*;

#[derive(Copy, Clone)]
enum ButtonId{
    PerformanceGraph,
    SwitchViewPoint,
    SwitchTexture,
}

#[derive(Copy, Clone)]
enum LabelId{
    Fps,
    Menu,
}

#[test]
fn create_and_use() -> Result<(), String> {
    let btn_performance_graph = Button::new(
        10, 
        20, 
        ButtonId::PerformanceGraph);
    let btn_switch_view_point = Button::new(
        10, 
        20, 
        ButtonId::SwitchViewPoint);
    let btn_switch_texture_pressed = Button::new(
        10, 
        20, 
        ButtonId::SwitchTexture);
    let lbl_fps = Label::new(
        10, 
        20, 
        LabelId::Fps);
    let lbl_menu = Label::new(
        10, 
        20, 
        LabelId::Menu);

    let vertical_layout =  VerticalLayout::<ButtonId, LabelId>::new(vec![
        GuiElement::Button(btn_performance_graph),
        GuiElement::Button(btn_switch_view_point),
        GuiElement::Button(btn_switch_texture_pressed),
        GuiElement::Label(lbl_fps),
        GuiElement::Label(lbl_menu),
    ]);

    let width = 800;
    let height = 600;
    let mut gui = Gui::new(width, height, vec![
        AlignedElement::new(Alignment::BottomRight, 20, 20, GuiElement::VerticalLayout(vertical_layout)), 
    ]);

    let res = gui.resize(400, 400);

    assert_eq!(res.len(), 5);

    Ok(())
}
