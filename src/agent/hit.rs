use bevy::prelude::*;
use derive_new::new;

#[derive(Debug, Clone, Copy, PartialEq, EntityEvent, new)]
pub struct DirectHit {
    entity: Entity,
    damage: u32,
}

impl DirectHit{
    pub fn get_damage(&self) -> u32 { self.damage }
}

impl Hit for DirectHit {
    fn get_entity(&self) -> Entity { self.entity }
}

#[derive(Debug, Clone, Copy, PartialEq, EntityEvent, new)]
pub struct HealHit {
    entity: Entity,
    heal: u32,
}

impl Hit for HealHit{
    fn get_entity(&self) -> Entity { self.entity }
}

impl HealHit{
    pub fn get_heal(&self) -> u32 { self.heal }
}


pub trait Hit{
    fn get_entity(&self) -> Entity;
} 