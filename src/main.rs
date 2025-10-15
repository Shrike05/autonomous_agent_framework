mod agent;

use crate::agent::agent::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AgentPlugin))
        .add_systems(Startup, init_test)
        .add_systems(Update, (log_hits, interact_enemy))
        .run();
}

#[derive(Debug, Clone, Copy, Component)]
struct Enemy;

fn init_test(mut commands: Commands) {
    commands.spawn((HP::new(10), Enemy));
}

fn interact_enemy(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    body_query: Query<Entity, (With<HP>, With<Enemy>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for body in body_query.iter() {
            commands.trigger(DirectHit::new(body, 1));
        }
    }
    if mouse.just_pressed(MouseButton::Right) {
        for body in body_query.iter() {
            commands.trigger(HealHit::new(body, 1));
        }
    }

}

fn log_hits(changed_query: Query<&HP, Changed<HP>>) {
    for hp in changed_query {
        println!("{}", hp.get_hp());
    }
}