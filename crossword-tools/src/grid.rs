use crate::puzzle::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
    pub number: u16,
    pub direction: Direction,
    pub indexes: Vec<usize>,
}

pub fn slots(width: usize, height: usize, is_block: impl Fn(usize) -> bool) -> Vec<Slot> {
    let mut slots = Vec::new();
    let mut number = 0;

    for index in 0..width * height {
        if is_block(index) {
            continue;
        }

        let row = index / width;
        let col = index % width;

        let starts_across =
            (col == 0 || is_block(index - 1)) && col + 1 < width && !is_block(index + 1);
        let starts_down =
            (row == 0 || is_block(index - width)) && row + 1 < height && !is_block(index + width);

        if !starts_across && !starts_down {
            continue;
        }

        number += 1;

        if starts_across {
            let end = (row + 1) * width;
            let indexes = (index..end).take_while(|&i| !is_block(i)).collect();
            slots.push(Slot { number, direction: Direction::Across, indexes });
        }

        if starts_down {
            let indexes = (index..width * height)
                .step_by(width)
                .take_while(|&i| !is_block(i))
                .collect();
            slots.push(Slot { number, direction: Direction::Down, indexes });
        }
    }

    slots
}
