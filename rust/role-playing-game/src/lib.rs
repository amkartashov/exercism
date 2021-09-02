// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            None
        } else {
            Some(Player{
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost > mana {
                0
            } else {
                self.mana = Some(mana - mana_cost);
                2*mana_cost
            }
        } else {
            self.health -= mana_cost;
            0
        }
    }
}
