use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::dynamics::Velocity;

mod ground;
pub use ground::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (movement, detect_ground));
	}
}

/// Movement speed of the agent
#[derive(Component)]
pub struct Speed(pub f32);

/// Direction the agent wants to go in
#[derive(Default, Component)]
pub struct MovementInput(pub Vec2);

fn movement(
	time: Res<Time>,
	mut q_agent: Query<(&mut Velocity, &OnGround, &MovementInput, &Speed)>,
) {
	for (mut vel, on_ground, input, speed) in &mut q_agent {
		let friction = match on_ground.0 {
			true => 64.0,
			false => 1.0,
		};

		let interp_t = 1.0 - (-friction * time.delta_seconds()).exp();

		let current_vel = vel.linvel.xz();

		let dir = input.0;

		let lacking = speed.0 - current_vel.dot(dir);
		vel.linvel += dir.extend(0.0).xzy() * lacking * interp_t;

		let extra = current_vel.dot(dir.perp());
		vel.linvel -= dir.perp().extend(0.0).xzy() * extra * interp_t;

		if dir == Vec2::ZERO {
			let linvel = vel.linvel;
			vel.linvel += -linvel.xz().extend(0.0).xzy() * interp_t;
		}
	}
}
