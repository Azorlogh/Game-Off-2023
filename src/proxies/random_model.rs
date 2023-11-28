use bevy::prelude::*;

pub struct RandomModelPlugin;
impl Plugin for RandomModelPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<RandomModel>()
			.add_systems(Update, lighting_replace_proxies);
	}
}

/// Give this component to an entity, and only one of its children will be shown
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct RandomModel;

pub fn lighting_replace_proxies(
	q_added_random_model: Query<&Children, Added<RandomModel>>,
	mut q_visibility: Query<&mut Visibility>,
	q_names: Query<&Name>,
) {
	for children in &q_added_random_model {
		let model_entities = children
			.iter()
			.filter(|e| {
				let is_blueprint_components = q_names
					.get(**e)
					.map(|name| name.ends_with("components"))
					.unwrap_or(false);
				!is_blueprint_components
			})
			.collect::<Vec<_>>();
		let vis_i = rand::random::<usize>() % model_entities.len();
		for (i, entity) in model_entities.into_iter().enumerate() {
			let mut vis = q_visibility.get_mut(*entity).unwrap();
			*vis = match i == vis_i {
				true => Visibility::Inherited,
				false => Visibility::Hidden,
			}
		}
	}
}
