pub enum ShirtColor {
    Red,
    Bule,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut bule_num = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Bule => bule_num += 1,
            }
        }
        if red_num > bule_num {
            ShirtColor::Red
        } else {
            ShirtColor::Bule
        }
    }
}