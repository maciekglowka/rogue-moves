use bevy::prelude::*;

use crate::units::{Unit, UnitState};

pub enum CommandType {
    AddAP(Entity, u8),
    RemoveAP(Entity),
    PauseUnit(Entity)
}

pub struct CommandEvent(pub CommandType);

pub struct CommandPlugin;
impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>();
        app.add_system(add_ap);
        app.add_system(remove_ap);
        app.add_system(pause_unit);
    }
}

fn add_ap(
    mut ev_command: EventReader<CommandEvent>,
    mut unit_query: Query<&mut Unit>
) {
    for ev in ev_command.iter() {
        match ev.0 {
            CommandType::AddAP(e, ap) => {
                if let Ok(mut unit) = unit_query.get_mut(e) {
                    unit.ap += ap;
                }
            },
            _ => ()
        }
    }
}

fn remove_ap(
    mut ev_command: EventReader<CommandEvent>,
    mut unit_query: Query<&mut Unit>
) {
    for ev in ev_command.iter() {
        match ev.0 {
            CommandType::RemoveAP(e) => {
                if let Ok(mut unit) = unit_query.get_mut(e) {
                    unit.ap = 0;
                }
            },
            _ => ()
        }
    }
}

fn pause_unit(
    mut ev_command: EventReader<CommandEvent>,
    mut unit_query: Query<&mut Unit>
) {
    for ev in ev_command.iter() {
        match ev.0 {
            CommandType::PauseUnit(e) => {
                if let Ok(mut unit) = unit_query.get_mut(e) {
                    unit.state = UnitState::Paused;
                    unit.ap = 0;
                }
            },
            _ => ()
        }
    }
}