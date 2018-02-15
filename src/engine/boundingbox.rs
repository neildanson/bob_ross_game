pub struct BoundingBox {
    x: i32,
    y: i32,
    x1: i32,
    y1: i32,
}

impl BoundingBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> BoundingBox {
        BoundingBox {
            x: x,
            y: y,
            x1: x + width as i32,
            y1: y + height as i32,
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        if (self.x > other.x && self.x < other.x1) || (self.x1 > other.x && self.x1 < other.x1) {
            if self.y > other.y && self.y < other.y1 {
                return true;
            }
            if self.y1 > other.y && self.y1 < other.y1 {
                return true;
            }
        }

        if (other.x > self.x && other.x < self.x1) || (other.x1 > self.x && other.x1 < self.x1) {
            if other.y > self.y && other.y < self.y1 {
                return true;
            }
            if other.y1 > self.y && other.y1 < self.y1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let bounding_box1 = BoundingBox::new(-160, -120, 320, 240);
        let bounding_box2 = BoundingBox::new(0, 0, 16, 16);
        assert_eq!(bounding_box1.intersects(&bounding_box2), true);
        assert_eq!(bounding_box2.intersects(&bounding_box1), true);
    }

    #[test]
    fn test2() {
        let bounding_box1 = BoundingBox::new(-160, -120, 320, 240);
        let bounding_box2 = BoundingBox::new(-100, -100, 16, 16);
        assert_eq!(bounding_box1.intersects(&bounding_box2), true);
        assert_eq!(bounding_box2.intersects(&bounding_box1), true);
    }
}
