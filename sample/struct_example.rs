pub struct Object
{
    pub width:u32,
    pub height:u32,
}

impl Object
{
    pub fn area(&self) -> u32
    {
        self.width * self.height
    }

    pub fn new(width: u32, height: u32) -> Object
    {
        Object{
            width: width,
            height: height,
        }
    }
}