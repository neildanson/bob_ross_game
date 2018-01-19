pub struct BoundingBox {
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> BoundingBox {
        BoundingBox {
            x: x,
            y: y,
            x1: x + width,
            y1: y + height,
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
        let bounding_box1 = BoundingBox::new(-160.0, -120.0, 320.0, 240.0);
        let bounding_box2 = BoundingBox::new(0.0, 0.0, 16.0, 16.0);
        assert_eq!(bounding_box1.intersects(&bounding_box2), true);
        assert_eq!(bounding_box2.intersects(&bounding_box1), true);
    }

    #[test]
    fn test2() {
        let bounding_box1 = BoundingBox::new(-160.0, -120.0, 320.0, 240.0);
        let bounding_box2 = BoundingBox::new(-100.0, -100.0, 16.0, 16.0);
        assert_eq!(bounding_box1.intersects(&bounding_box2), true);
        assert_eq!(bounding_box2.intersects(&bounding_box1), true);
    }
}
