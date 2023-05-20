use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::PlayerMove;

pub(super) fn move_player(
    mut player_query: Query<(&mut Velocity, &PlayerMove)>,
    keys: Res<Input<KeyCode>>,
    mut stopped_player: Local<bool>,
    mut x_velocity_when_stopped: Local<f32>,
    mut counter: Local<f32>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    let Ok((mut velocity, player)) = player_query.get_single_mut() else {
      return;
  };

    if keys.pressed(KeyCode::A) {
        velocity.linvel = Vec2::new(
            (velocity.linvel.x - player.speed).max(-player.max_speed),
            velocity.linvel.y,
        );
        *stopped_player = false;
    } else if keys.pressed(KeyCode::D) {
        velocity.linvel = Vec2::new(
            (velocity.linvel.x + player.speed).min(player.max_speed),
            velocity.linvel.y,
        );
        *stopped_player = false;
    } else {
        if *stopped_player == true {
            let mut stopped_amount = 1.0 - (*counter / player.seconds_to_stop_after_key_release);

            if stopped_amount > 0.0 {
                *counter += delta_time;
            } else {
                stopped_amount = 0.0
            }

            velocity.linvel =
                Vec2::new(*x_velocity_when_stopped * stopped_amount, velocity.linvel.y);
        } else {
            *counter = 0.0;
            *stopped_player = true;
            *x_velocity_when_stopped = velocity.linvel.x;
        }
    }
}
