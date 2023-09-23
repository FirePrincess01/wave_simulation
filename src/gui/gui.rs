

use super::AlignedElement;

enum MouseEvent {
    Pressed(glam::Vec2),
    Released(glam::Vec2),
    Moved(glam::Vec2),
}

pub enum ElementId<ButtonId, LabelId>
{
    Button(ButtonId),
    Label(LabelId),
}

pub struct ChangePositionEvent<ButtonId, LabelId>{
    element_id: ElementId<ButtonId, LabelId>,
    x: u32,
    y: u32,
}

impl<ButtonId, LabelId> ChangePositionEvent<ButtonId, LabelId> {
    pub fn new_button(button_id: ButtonId,
        x: u32,
        y: u32) -> Self 
    {
        let element_id = ElementId::<ButtonId, LabelId>::Button(button_id);

        Self {
            element_id,
            x,
            y,
        }
    }

    pub fn new_label(label_id: LabelId,
        x: u32,
        y: u32) -> Self 
    {
        let element_id = ElementId::<ButtonId, LabelId>::Label(label_id);

        Self {
            element_id,
            x,
            y,
        }
    }
}

pub struct ButtonPressedEvent<ElementId>{
    element_id: ElementId,
}


pub struct Gui<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    width: u32,
    height: u32,

    elements: Vec<AlignedElement<ButtonId, LabelId>>,
}

impl<ButtonId, LabelId> Gui<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    pub fn new(width: u32, height: u32, elements: Vec<AlignedElement<ButtonId, LabelId>>) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Vec<ChangePositionEvent<ButtonId, LabelId>> {
        self.width = width;
        self.height = height;
        
        let mut res = Vec::<ChangePositionEvent<ButtonId, LabelId>>::new();

        for elem in &mut self.elements {
            elem.resize(self.width, self.height, &mut res);
        }

        res
    }

    pub fn mouse_event(&self, mouse_event: &MouseEvent) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        
        (false, None)
    }
}