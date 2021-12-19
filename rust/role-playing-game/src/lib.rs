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
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player { health: 100, mana: Some(100), level: self.level });
            } else {
                return Some(Player { health: 100, mana: None, level: self.level });
            }
        } else {
            return None
        }
    }
    
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut damage: u32 = 0;
        if self.mana.is_none() {
            if mana_cost > self.health {
                self.health = 0
            } else { 
                self.health -= mana_cost;
            }
        } else {
            let cur_mana: u32 = self.mana.unwrap();
            if cur_mana > mana_cost {
                self.mana = Some(cur_mana - mana_cost);
                damage = mana_cost * 2;
            }
        }
        damage
    }
}
