use bevy::{
	prelude::*,
	render::mesh::{MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues},
};

pub(crate) trait MeshExt {
	fn transform(&mut self, transform: Transform);
	fn transformed(&self, transform: Transform) -> Mesh;
	fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]>;
	fn search_in_children<'a>(
		parent: Entity,
		children: &'a Query<&Children>,
		meshes: &'a Assets<Mesh>,
		mesh_handles: &'a Query<&Handle<Mesh>>,
	) -> Vec<(Entity, &'a Mesh)>;
}

impl MeshExt for Mesh {
	fn transform(&mut self, transform: Transform) {
		for coords in self.read_coords_mut(Mesh::ATTRIBUTE_POSITION.clone()) {
			let vec3 = (*coords).into();
			let transformed = transform.transform_point(vec3);
			*coords = transformed.into();
		}
		for normal in self.read_coords_mut(Mesh::ATTRIBUTE_NORMAL.clone()) {
			let vec3 = (*normal).into();
			let transformed = transform.rotation.mul_vec3(vec3);
			*normal = transformed.into();
		}
	}

	fn transformed(&self, transform: Transform) -> Mesh {
		let mut mesh = self.clone();
		mesh.transform(transform);
		mesh
	}

	fn read_coords_mut(&mut self, id: impl Into<MeshVertexAttributeId>) -> &mut Vec<[f32; 3]> {
		// Guaranteed by Bevy for the current usage
		match self
			.attribute_mut(id)
			.expect("Failed to read unknown mesh attribute")
		{
			VertexAttributeValues::Float32x3(values) => values,
			// Guaranteed by Bevy for the current usage
			_ => unreachable!(),
		}
	}

	fn search_in_children<'a>(
		parent: Entity,
		children_query: &'a Query<&Children>,
		meshes: &'a Assets<Mesh>,
		mesh_handles: &'a Query<&Handle<Mesh>>,
	) -> Vec<(Entity, &'a Mesh)> {
		if let Ok(children) = children_query.get(parent) {
			let mut result: Vec<_> = children
				.iter()
				.filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
				.map(|(entity, mesh_handle)| {
					(
						entity,
						meshes
							.get(mesh_handle)
							.expect("Failed to get mesh from handle"),
					)
				})
				.map(|(entity, mesh)| {
					assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
					(entity, mesh)
				})
				.collect();
			let mut inner_result = children
				.iter()
				.flat_map(|entity| {
					Self::search_in_children(*entity, children_query, meshes, mesh_handles)
				})
				.collect();
			result.append(&mut inner_result);
			result
		} else {
			Vec::new()
		}
	}
}

pub trait VecExt {
    fn sorted(self) -> Self;
}

impl<T> VecExt for Vec<T>
where
    T: std::cmp::Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}