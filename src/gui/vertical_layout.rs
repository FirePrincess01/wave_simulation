
use super::GuiElement;
use super::ChangePositionEvent;

pub struct VerticalLayout<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    elements: Vec<GuiElement<ButtonId, LabelId>>,
}

impl<ButtonId, LabelId> VerticalLayout<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    pub fn new(elements: Vec<GuiElement<ButtonId, LabelId>>) -> Self 
    {
        Self {
            elements
        }
    }

    pub fn resize(&self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<ButtonId, LabelId>>)
    {
        for element in &self.elements {
            match element {
                GuiElement::Button(button) => {
                    let button_id = button.id();
                    let event = ChangePositionEvent::<ButtonId, LabelId>::new_button(button_id, abs_x, abs_y);
                    res.push(event);
                }
                GuiElement::Label(label) => {
                    let label_id = label.id();
                    let event = ChangePositionEvent::<ButtonId, LabelId>::new_label(label_id, abs_x, abs_y);
                    res.push(event);
                }
                GuiElement::VerticalLayout(vertical_layout) => {
                    vertical_layout.resize(abs_x, abs_y, res);
                }
            }
        }
    }

}
