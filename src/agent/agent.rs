use bevy::prelude::*;
use super::hp::*;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (new_hp_body, remove_hp_body, tick_invincibility_timer));
        app.add_observer(recieve_hit);
        app.add_observer(recieve_heal);
    }
}

