use bevy_ecs::world::EntityWorldMut;
use bevy_mesh::Mesh;
use bevy_reflect::TypePath;
use bevy_transform::components::Transform;
use bevy_asset::LoadContext;
use bevy_pbr::{DirectionalLight, Material, PointLight, SpotLight, StandardMaterial};
use gltf::{khr_lights_punctual::Light, Node, Primitive};

use crate::*;

pub trait GltfTrait: Send+Sync+'static+TypePath+Clone {
        /// The extensions used by the asset loader
        const EXTENSIONS: &'static [&'static str] = &["gltf", "glb"];
        /// Actual Material attached to Entities
        type Material:Material;
        /// convert materials
        fn convert_material (convert:GltfTraitMaterial) -> Self::Material;
        /// Edit the app
        fn on_app (_app:&mut App){}
        /// Edit the entity or [Transform] of any light
        fn on_light_parent (_edit:GltfTraitParent){}
        /// Edit the entity or component of a [DirectionalLight]
        fn edit_directional_light (_edit:GltfTraitLight<DirectionalLight>){}
        /// Edit the entity or component of a [PointLight]
        fn edit_point_light (_edit:GltfTraitLight<PointLight>){}
        /// Edit the entity or component of a [SpotLight]
        fn edit_spot_light (_edit:GltfTraitLight<SpotLight>){}        
        /// Edit the entity or [Transform] of a set of meshes 
        fn on_mesh_parent (_edit:GltfTraitParent){}      
        /// Edit the entity or [Transform] of a set of skinned meshes 
        fn on_skinned_mesh_parent (_edit:GltfTraitParent){}
        /// Edit the entity of a skinned mesh 
        fn on_skinned_mesh (_edit:GltfTraitEntity){}
        /// Edit the entity of a mesh
        fn on_mesh (_edit:GltfTraitEntity){}
        /// Edit meshes
        fn edit_mesh (_edit:GltfTraitMesh){}        

    }
    pub struct GltfTraitDefault;
    impl GltfTrait for () {
        type Material = StandardMaterial;        
        fn convert_material (convert:GltfTraitMaterial) -> Self::Material {
            convert.material
        }
    }

/* --------------------------------- Helpers -------------------------------- */
    
    /// Struct to simplify parameters of the [GltfTrait] light parent method
    pub struct GltfTraitMaterial <'a,'b> {
        pub context: &'b LoadContext<'a>,
        pub material: StandardMaterial,
        pub raw:&'b gltf::Material<'a>
    }

    /// Struct to simplify parameters of the [GltfTrait] light parent method
    pub struct GltfTraitEntity <'a,'b> {
        pub context: &'b LoadContext<'a>,
        pub entity: &'b mut EntityWorldMut<'a>,
        pub node: &'b Node<'a>,
        pub mesh: &'b gltf::Mesh<'a>,
        pub primitive: &'b Primitive<'a>
    }

    /// Struct to simplify parameters of the [GltfTrait] light parent method
    pub struct GltfTraitMesh <'a,'b> {
        pub context: &'b LoadContext<'a>,
        pub mesh: &'b mut Mesh,
        pub raw: &'b Primitive<'a>
    }

    /// Struct to simplify parameters of the [GltfTrait] light parent method
    pub struct GltfTraitParent <'a,'b> {
        pub context: &'b LoadContext<'a>,
        pub entity: &'b mut EntityWorldMut<'a>,
        pub transform: &'b mut Transform,
        pub node:&'b Node<'a>
    }

    /// Struct to simplify parameters of [GltfTrait]s light methods
    pub struct GltfTraitLight <'a,'b, L> {
        pub context: &'b LoadContext<'a>,
        pub entity: &'b mut EntityWorldMut<'a>,
        pub component: &'b mut L,
        pub node:&'b Node<'a>,
        pub raw: &'b Light<'b>
    }
    impl <'a,'b,L> GltfTraitLight <'a,'b, L> {
        pub fn new(
            context: &'b LoadContext<'a>,
            entity: &'b mut EntityWorldMut<'a>,
            component: &'b mut L,
            node:&'b Node<'a>,
            raw: &'b Light<'a>
        ) -> Self { Self {
            context,
            entity,
            component,
            node,
            raw
        }}
    }

/* ------------------------------- Traited OG ------------------------------- */

