use crate::food::SpawnFood;

fn setup_food(mut event_writer: EventWriter<SpawnFood>, pos: Vec3) {
	event_writer.send(SpawnFood {
		name: String::from("Apple"),
		model: String::from("models/foods/glb/Apple.glb#Scene0"),
		stats: FoodStats {
			hydration: 0.5,
			glucose: 0.5,
		},
		position: pos,
		scale_factor: 0.1,
	})
}
