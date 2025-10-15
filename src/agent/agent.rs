use std::time::Duration;

use bevy::prelude::*;
use derive_new::new;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (new_hp_body, remove_hp_body, tick_invincibility_timer));
        app.add_observer(recieve_hit);
        app.add_observer(recieve_heal);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Component)]
pub struct HP {
    max_hp: u32,
    invincibility_time: f32,
    hp: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Component)]
struct InvincibilityTimer(Timer);

impl HP {
    pub fn new(hp: u32) -> HP {
        HP {
            max_hp: hp,
            hp: hp,
            invincibility_time: 0.7,
        }
    }

    pub fn get_max_hp(&self) -> u32 {self.max_hp}
    pub fn get_hp(&self) -> u32 {self.hp}
    pub fn get_invincibility_time(&self) -> f32 {self.invincibility_time}

    pub fn take_damage(&mut self, dmg: u32) {
        self.hp = if dmg <= self.hp { self.hp - dmg } else { 0 }
    }

    pub fn recieve_heal(&mut self, heal: u32) {
        self.hp = self.get_max_hp().min(self.hp + heal);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EntityEvent, new)]
pub struct DirectHit {
    entity: Entity,
    damage: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, EntityEvent, new)]
pub struct HealHit {
    entity: Entity,
    heal: u32,
}

fn recieve_hit(hit: On<DirectHit>, mut hittable_bodies: Query<(&mut HP, &mut InvincibilityTimer)>) {
    let Ok((mut hp, mut timer)) = hittable_bodies.get_mut(hit.entity) else {
        return;
    };

    if !timer.0.is_finished(){
        return;
    }

    hp.take_damage(hit.damage);
    timer.0 = Timer::from_seconds(hp.get_invincibility_time(), TimerMode::Once);
}

fn recieve_heal(hit: On<HealHit>, mut hittable_bodies: Query<&mut HP>) {
    let Ok(mut hp) = hittable_bodies.get_mut(hit.entity) else {
        return;
    };

    hp.recieve_heal(hit.heal);
}


fn tick_invincibility_timer(
    mut timers_query: Query<&mut InvincibilityTimer>,
    time: Res<Time>
){
    for mut timer in timers_query.iter_mut(){
        if !timer.0.is_finished(){
            timer.0.tick(time.delta());
        }
    }
}

fn new_hp_body(
    new_bodies_query: Query<Entity, Added<HP>>,
    mut commands: Commands
){
    for entity in new_bodies_query{
        commands.entity(entity).insert(InvincibilityTimer(Timer::new(Duration::ZERO, TimerMode::Once)));
    }
}

fn remove_hp_body(
    removed_bodies_query: Query<Entity, (With<InvincibilityTimer>, Without<HP>)>,
    mut commands: Commands
){
    for entity in removed_bodies_query{
        commands.entity(entity).remove::<InvincibilityTimer>();
    }
}