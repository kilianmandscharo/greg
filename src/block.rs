use crate::time_of_day::TimeOfDay;

#[derive(Clone, Debug)]
pub struct Block {
    pub start: TimeOfDay,
    pub end: TimeOfDay,
    pub title: String,
    pub description: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Block {
    pub fn new(
        start: TimeOfDay,
        end: TimeOfDay,
        title: String,
        description: String,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            start,
            end,
            title,
            description,
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Block) -> bool {
        !(self.y + self.height < other.y || self.y > other.y + other.height)
            && !(self.x + self.width < other.x || self.x > other.x + other.width)
    }

    pub fn intersects_any(&self, others: &[Block]) -> bool {
        others.iter().any(|block| block.intersects(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::time_of_day::TimeOfDay;

    use super::Block;

    #[test]
    fn test_intersection() {
        let test_cases = vec![
            // (
            //     Block::new(
            //         TimeOfDay::new(13, 0),
            //         TimeOfDay::new(15, 0),
            //         "Lunch".to_string(),
            //         "Lunch at the café".to_string(),
            //         30,
            //         0,
            //         20,
            //         2,
            //     ),
            //     Block::new(
            //         TimeOfDay::new(14, 0),
            //         TimeOfDay::new(16, 0),
            //         "Dentist".to_string(),
            //         "Half-yearly dentis appointment".to_string(),
            //         40,
            //         0,
            //         20,
            //         2,
            //     ),
            //     true,
            // ),
            (
                Block::new(
                    TimeOfDay::new(13, 0),
                    TimeOfDay::new(15, 0),
                    "Lunch".to_string(),
                    "Lunch at the café".to_string(),
                    30,
                    0,
                    20,
                    2,
                ),
                Block::new(
                    TimeOfDay::new(14, 0),
                    TimeOfDay::new(16, 0),
                    "Dentist".to_string(),
                    "Half-yearly dentis appointment".to_string(),
                    40,
                    3,
                    20,
                    2,
                ),
                false,
            ),
        ];

        for test in test_cases {
            let (block, other, intersect) = test;
            assert_eq!(block.intersects(&other), intersect);
        }
    }
}
