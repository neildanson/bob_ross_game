use engine::BoundingBox;
use engine::SpriteSheet;

pub trait Entity {
    fn position(&self) -> (i32, i32);
    //fn boundingbox() -> BoundingBox; 
    //fn spritesheet() -> SpriteSheet;
    fn animation_index(&self) -> usize;
}