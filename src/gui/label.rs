
pub struct Label<LabelId> 
    where LabelId: Copy
{
    width: u32,
    height: u32,
    label_id: LabelId,
}

impl<LabelId> Label<LabelId>
    where LabelId: Copy
{
    pub fn new(width: u32,
        height: u32,
        label_id: LabelId) -> Self
    {
        Self{ 
            width, 
            height, 
            label_id,
        }
    }

    pub fn id(&self) -> LabelId {
        self.label_id
    }
}