use texture_region::TextureRegion;

trait Component {
}

pub struct Position{ pub x: f32, pub y: f32 }
pub struct Velocity { pub x: f32, pub y: f32 }
pub struct Sized { pub width: f32, pub height: f32 }
pub struct TextureRegioned<'a> (pub &'a TextureRegion);

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            x, y
        }
        
    }
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity {
            x, y
        }
        
    }
}
impl Sized {
    pub fn new(width: f32, height: f32) -> Sized {
        Sized {
            width, height
        }
        
    }
}
impl<'a> TextureRegioned<'a> {
    pub fn new(t: &'a TextureRegion) -> TextureRegioned {
        TextureRegioned(t)
        
    }
}

impl Component for Position {
}
impl Component for Velocity {}
impl Component for Sized {}
