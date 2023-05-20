use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::platform::Platform;

use super::{PlayerGroundSensor, PlayerJump, PlayerLandedOnEvent, PlayerLegs};

pub(super) fn ground_player(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<Entity, With<PlayerGroundSensor>>,
    mut player_jump_query: Query<&mut PlayerJump, Without<PlayerGroundSensor>>,
    mut ev_landed: EventWriter<PlayerLandedOnEvent>,
    platforms: Query<Entity, With<Platform>>,
) {
    let Ok(sensor) = sensor_query.get_single() else {
      return;
  };
    let Ok(mut player_jump) = player_jump_query.get_single_mut() else {
      return;
  };

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor || h2 == &sensor {
                for platform in platforms.iter() {
                    if platform == *h2 {
                        ev_landed.send(PlayerLandedOnEvent(*h2));
                    }

                    if platform == *h1 {
                        ev_landed.send(PlayerLandedOnEvent(*h1));
                    }
                }

                player_jump.grounded = true;
            }
        }

        if let CollisionEvent::Stopped(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor || h2 == &sensor {
                player_jump.grounded = false;
            }
        }
    }
}

pub(super) fn jump_player(
    mut player_query: Query<(&mut Velocity, &mut PlayerJump), Without<PlayerLegs>>,
    mut player_legs_query: Query<&mut Transform, (With<PlayerLegs>, Without<PlayerJump>)>,
    keys: Res<Input<KeyCode>>,
    mut time_since_jump: Local<f32>,
    time: Res<Time>,
) {
    let Ok((mut velocity, mut player_jump)) = player_query.get_single_mut() else {
      return;
  };
    let Ok(mut legs_transform) = player_legs_query.get_single_mut() else {
      return;
  };

    if keys.just_pressed(KeyCode::W) && player_jump.grounded {
        velocity.linvel.y = player_jump.jump_force;
        player_jump.grounded = false;

        *time_since_jump = 0.0;
    } else {
        *time_since_jump += time.delta_seconds();
        legs_transform.scale.y = 2.0;
    }

    if *time_since_jump < 0.2 {
        legs_transform.scale.y = 1.8;
    }
}
