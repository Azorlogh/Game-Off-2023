use bevy::prelude::*;

use super::health::Hit;

pub struct HitEffectPlugin;
impl Plugin for HitEffectPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (handle_take_hit, update_hit_effect));
	}
}

#[derive(Component)]
pub struct HitEffect {
	base_material: Handle<StandardMaterial>,
	base_color: Color,
	remaining: f32,
}

const FX_TIME: f32 = 0.2;

fn handle_take_hit(
	mut cmds: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
	q_children: Query<&Children>,
	mut q_hit_fx: Query<&mut HitEffect>,
	mut q_material: Query<&mut Handle<StandardMaterial>>,
	mut ev_hit: EventReader<Hit>,
) {
	let mut try_replace_material = |entity: Entity| {
		// if the effect is already present, just reset it
		if let Ok(mut fx) = q_hit_fx.get_mut(entity) {
			fx.remaining = FX_TIME;
			return;
		}
		// otherwise add it
		if let Ok(mut handle) = q_material.get_mut(entity) {
			let mut mat = materials.get(&handle).unwrap().clone();

			// save the original material handle
			let mut original_handle = handle.clone();
			original_handle.make_strong(&materials);
			cmds.entity(entity).insert(HitEffect {
				base_material: original_handle,
				base_color: mat.base_color,
				remaining: FX_TIME,
			});

			// attach a new temporary material for the effect

			mat.base_color = Color::RED;
			*handle = materials.add(mat);
		}
	};
	for ev in ev_hit.iter() {
		try_replace_material(ev.target);
		for child_e in q_children.iter_descendants(ev.target) {
			try_replace_material(child_e);
		}
	}
}

fn update_hit_effect(
	mut cmds: Commands,
	time: Res<Time>,
	mut q_fx: Query<(Entity, &mut HitEffect, &mut Handle<StandardMaterial>)>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for (entity, mut fx, mut mat_handle) in &mut q_fx {
		let new_remaining = fx.remaining - time.delta_seconds();
		if new_remaining <= 0.0 {
			cmds.entity(entity).remove::<HitEffect>();
			*mat_handle = fx.base_material.clone();
		} else {
			let mat = materials.get_mut(&mat_handle).unwrap();
			let t = new_remaining / FX_TIME;
			mat.base_color = Color::rgb(1.0, 0.0, 0.0) * t + fx.base_color * (1.0 - t);
			fx.remaining = new_remaining;
		}
	}
}
