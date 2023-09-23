
pub struct Button<ButtonId> 
    where ButtonId: Copy
{
    width: u32,
    height: u32,
    button_id: ButtonId,
}

impl<ButtonId> Button<ButtonId>
    where ButtonId: Copy
{
    pub fn new(width: u32,
        height: u32,
        button_id: ButtonId) -> Self
    {
        Self{ 
            width, 
            height, 
            button_id
        }
    }

    pub fn id(&self) -> ButtonId {
        self.button_id
    }
}