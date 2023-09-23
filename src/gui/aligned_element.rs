
use super::GuiElement;
use super::ChangePositionEvent;

pub enum Alignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct AlignedElement<ButtonId, LabelId>
where LabelId: Copy,
    ButtonId: Copy,
{
    alignment: Alignment,
    x: u32,
    y: u32,
    element: GuiElement<ButtonId, LabelId>,
}

impl<ButtonId, LabelId> AlignedElement<ButtonId, LabelId>
where LabelId: Copy,
    ButtonId: Copy,
{
    pub fn new(alignment: Alignment, x: u32, y:u32, element: GuiElement<ButtonId, LabelId>) -> Self 
    {
        Self {
            alignment,
            x,
            y,
            element
        }
    }

    fn calculate_absolute_position(&self, width: u32, height: u32) -> (u32, u32) {
        match self.alignment {
            Alignment::TopLeft =>     (width + self.x, height + self.y),
            Alignment::TopRight =>    (width + self.x, height - self.y),
            Alignment::BottomLeft =>  (width - self.x, height + self.y),
            Alignment::BottomRight => (width - self.x, height - self.y),
        }
    }

    pub fn resize(&self, width: u32, height: u32, res: &mut Vec::<ChangePositionEvent<ButtonId, LabelId>>)
    {
        let (abs_x, abs_y) = self.calculate_absolute_position(width, height);

        match &self.element {
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
