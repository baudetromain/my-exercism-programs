// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player
{
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player
{
    pub fn revive(&self) -> Option<Player>
    {
        match self.health
        {
            0 => Some(Player
            {
                health: 100,
                mana: match self.mana
                {
                    Some(_) => Some(100),
                    None => None
                },
                level: self.level,
            }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32
    {
        match self.mana
        {
            Some(mana) =>
            {
                if mana < mana_cost
                {
                    0
                }
                else
                {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    2 * mana_cost
                }
            },
            None =>
            {
                self.health = {
                    if self.health < mana_cost
                    {
                        0
                    }
                    else
                    {
                        self.health - mana_cost
                    }
                };
                0
            }
        }
    }
}
