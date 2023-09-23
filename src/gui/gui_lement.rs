
use super::Button;
use super::Label;
use super::VerticalLayout;

pub enum GuiElement<ButtonId, LabelId>
where LabelId: Copy,
    ButtonId: Copy,
{
    Button(Button<ButtonId>),
    Label(Label<LabelId>),
    VerticalLayout(VerticalLayout<ButtonId, LabelId>),
    // HorizontalLayout(GuiHorizontalLayout),
}