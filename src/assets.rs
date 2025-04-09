//! Representation of assets present in a glTF file

#[cfg(feature = "bevy_animation")]
use bevy_animation::AnimationClip;
use bevy_asset::{Asset, Handle};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_mesh::{skinning::SkinnedMeshInverseBindposes, Mesh};
use bevy_platform_support::collections::HashMap;
use bevy_reflect::{prelude::ReflectDefault, Reflect, TypePath};
use bevy_scene::Scene;

use crate::{ext::GltfTrait, GltfAssetLabel};

/// Representation of a loaded glTF file.
#[derive(Asset, Debug, TypePath)]
pub struct Gltf <G:GltfTrait> {
    /// All scenes loaded from the glTF file.
    pub scenes: Vec<Handle<Scene>>,
    /// Named scenes loaded from the glTF file.
    pub named_scenes: HashMap<Box<str>, Handle<Scene>>,
    /// All meshes loaded from the glTF file.
    pub meshes: Vec<Handle<GltfMesh<G>>>,
    /// Named meshes loaded from the glTF file.
    pub named_meshes: HashMap<Box<str>, Handle<GltfMesh<G>>>,
    /// All materials loaded from the glTF file.
    pub materials: Vec<Handle<G::Material>>,
    /// Named materials loaded from the glTF file.
    pub named_materials: HashMap<Box<str>, Handle<G::Material>>,
    /// All nodes loaded from the glTF file.
    pub nodes: Vec<Handle<GltfNode<G>>>,
    /// Named nodes loaded from the glTF file.
    pub named_nodes: HashMap<Box<str>, Handle<GltfNode<G>>>,
    /// All skins loaded from the glTF file.
    pub skins: Vec<Handle<GltfSkin<G>>>,
    /// Named skins loaded from the glTF file.
    pub named_skins: HashMap<Box<str>, Handle<GltfSkin<G>>>,
    /// Default scene to be displayed.
    pub default_scene: Option<Handle<Scene>>,
    /// All animations loaded from the glTF file.
    #[cfg(feature = "bevy_animation")]
    pub animations: Vec<Handle<AnimationClip>>,
    /// Named animations loaded from the glTF file.
    #[cfg(feature = "bevy_animation")]
    pub named_animations: HashMap<Box<str>, Handle<AnimationClip>>,
    /// The gltf root of the gltf asset, see <https://docs.rs/gltf/latest/gltf/struct.Gltf.html>. Only has a value when `GltfLoaderSettings::include_source` is true.
    pub source: Option<gltf::Gltf>,
}

/// A glTF mesh, which may consist of multiple [`GltfPrimitives`](GltfPrimitive)
/// and an optional [`GltfExtras`].
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-mesh).
#[derive(Asset, Clone, TypePath)]
pub struct GltfMesh <G:GltfTrait> {
    /// Index of the mesh inside the scene
    pub index: usize,
    /// Computed name for a mesh - either a user defined mesh name from gLTF or a generated name from index
    pub name: String,
    /// Primitives of the glTF mesh.
    pub primitives: Vec<GltfPrimitive<G>>,
    /// Additional data.
    pub extras: Option<GltfExtras>,
}

impl <G:GltfTrait> GltfMesh <G> {
    /// Create a mesh extracting name and index from glTF def
    pub fn new(
        mesh: &gltf::Mesh,
        primitives: Vec<GltfPrimitive<G>>,
        extras: Option<GltfExtras>,
    ) -> Self {
        Self {
            index: mesh.index(),
            name: if let Some(name) = mesh.name() {
                name.to_string()
            } else {
                format!("GltfMesh{}", mesh.index())
            },
            primitives,
            extras,
        }
    }

    /// Subasset label for this mesh within the gLTF parent asset.
    pub fn asset_label(&self) -> GltfAssetLabel {
        GltfAssetLabel::Mesh(self.index)
    }
}

/// A glTF node with all of its child nodes, its [`GltfMesh`],
/// [`Transform`](bevy_transform::prelude::Transform), its optional [`GltfSkin`]
/// and an optional [`GltfExtras`].
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-node).
#[derive(Asset, Debug, Clone, TypePath)]
pub struct GltfNode <G:GltfTrait> {
    /// Index of the node inside the scene
    pub index: usize,
    /// Computed name for a node - either a user defined node name from gLTF or a generated name from index
    pub name: String,
    /// Direct children of the node.
    pub children: Vec<Handle<GltfNode<G>>>,
    /// Mesh of the node.
    pub mesh: Option<Handle<GltfMesh<G>>>,
    /// Skin of the node.
    pub skin: Option<Handle<GltfSkin<G>>>,
    /// Local transform.
    pub transform: bevy_transform::prelude::Transform,
    /// Is this node used as an animation root
    #[cfg(feature = "bevy_animation")]
    pub is_animation_root: bool,
    /// Additional data.
    pub extras: Option<GltfExtras>,
}

impl <G:GltfTrait> GltfNode <G> {
    /// Create a node extracting name and index from glTF def
    pub fn new(
        node: &gltf::Node,
        children: Vec<Handle<GltfNode<G>>>,
        mesh: Option<Handle<GltfMesh<G>>>,
        transform: bevy_transform::prelude::Transform,
        skin: Option<Handle<GltfSkin<G>>>,
        extras: Option<GltfExtras>,
    ) -> Self {
        Self {
            index: node.index(),
            name: if let Some(name) = node.name() {
                name.to_string()
            } else {
                format!("GltfNode{}", node.index())
            },
            children,
            mesh,
            transform,
            skin,
            #[cfg(feature = "bevy_animation")]
            is_animation_root: false,
            extras,
        }
    }

    /// Create a node with animation root mark
    #[cfg(feature = "bevy_animation")]
    pub fn with_animation_root(self, is_animation_root: bool) -> Self {
        Self {
            is_animation_root,
            ..self
        }
    }

    /// Subasset label for this node within the gLTF parent asset.
    pub fn asset_label(&self) -> GltfAssetLabel {
        GltfAssetLabel::Node(self.index)
    }
}

/// Part of a [`GltfMesh`] that consists of a [`Mesh`], an optional [`StandardMaterial`] and [`GltfExtras`].
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-mesh-primitive).
#[derive(Asset, Debug, Clone, TypePath)]
pub struct GltfPrimitive <G:GltfTrait> {
    /// Index of the primitive inside the mesh
    pub index: usize,
    /// Index of the parent [`GltfMesh`] of this primitive
    pub parent_mesh_index: usize,
    /// Computed name for a primitive - either a user defined primitive name from gLTF or a generated name from index
    pub name: String,
    /// Topology to be rendered.
    pub mesh: Handle<Mesh>,
    /// Material to apply to the `mesh`.
    pub material: Option<Handle<G::Material>>,
    /// Additional data.
    pub extras: Option<GltfExtras>,
    /// Additional data of the `material`.
    pub material_extras: Option<GltfExtras>,
}

impl <G:GltfTrait> GltfPrimitive <G> {
    /// Create a primitive extracting name and index from glTF def
    pub fn new(
        gltf_mesh: &gltf::Mesh,
        gltf_primitive: &gltf::Primitive,
        mesh: Handle<Mesh>,
        material: Option<Handle<G::Material>>,
        extras: Option<GltfExtras>,
        material_extras: Option<GltfExtras>,
    ) -> Self {
        GltfPrimitive {
            index: gltf_primitive.index(),
            parent_mesh_index: gltf_mesh.index(),
            name: {
                let mesh_name = gltf_mesh.name().unwrap_or("Mesh");
                if gltf_mesh.primitives().len() > 1 {
                    format!("{}.{}", mesh_name, gltf_primitive.index())
                } else {
                    mesh_name.to_string()
                }
            },
            mesh,
            material,
            extras,
            material_extras,
        }
    }

    /// Subasset label for this primitive within its parent [`GltfMesh`] within the gLTF parent asset.
    pub fn asset_label(&self) -> GltfAssetLabel {
        GltfAssetLabel::Primitive {
            mesh: self.parent_mesh_index,
            primitive: self.index,
        }
    }
}

/// A glTF skin with all of its joint nodes, [`SkinnedMeshInversiveBindposes`](bevy_mesh::skinning::SkinnedMeshInverseBindposes)
/// and an optional [`GltfExtras`].
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-skin).
#[derive(Asset, Debug, Clone, TypePath)]
pub struct GltfSkin <G:GltfTrait> {
    /// Index of the skin inside the scene
    pub index: usize,
    /// Computed name for a skin - either a user defined skin name from gLTF or a generated name from index
    pub name: String,
    /// All the nodes that form this skin.
    pub joints: Vec<Handle<GltfNode<G>>>,
    /// Inverse-bind matrices of this skin.
    pub inverse_bind_matrices: Handle<SkinnedMeshInverseBindposes>,
    /// Additional data.
    pub extras: Option<GltfExtras>,
}

impl <G:GltfTrait> GltfSkin <G> {
    /// Create a skin extracting name and index from glTF def
    pub fn new(
        skin: &gltf::Skin,
        joints: Vec<Handle<GltfNode<G>>>,
        inverse_bind_matrices: Handle<SkinnedMeshInverseBindposes>,
        extras: Option<GltfExtras>,
    ) -> Self {
        Self {
            index: skin.index(),
            name: if let Some(name) = skin.name() {
                name.to_string()
            } else {
                format!("GltfSkin{}", skin.index())
            },
            joints,
            inverse_bind_matrices,
            extras,
        }
    }

    /// Subasset label for this skin within the gLTF parent asset.
    pub fn asset_label(&self) -> GltfAssetLabel {
        GltfAssetLabel::Skin(self.index)
    }
}

/// Additional untyped data that can be present on most glTF types at the primitive level.
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-extras).
#[derive(Clone, Debug, Reflect, Default, Component)]
#[reflect(Component, Clone, Default, Debug)]
pub struct GltfExtras {
    /// Content of the extra data.
    pub value: String,
}

impl From<&serde_json::value::RawValue> for GltfExtras {
    fn from(value: &serde_json::value::RawValue) -> Self {
        GltfExtras {
            value: value.get().to_string(),
        }
    }
}

/// Additional untyped data that can be present on most glTF types at the scene level.
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-extras).
#[derive(Clone, Debug, Reflect, Default, Component)]
#[reflect(Component, Clone, Default, Debug)]
pub struct GltfSceneExtras {
    /// Content of the extra data.
    pub value: String,
}

/// Additional untyped data that can be present on most glTF types at the mesh level.
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-extras).
#[derive(Clone, Debug, Reflect, Default, Component)]
#[reflect(Component, Clone, Default, Debug)]
pub struct GltfMeshExtras {
    /// Content of the extra data.
    pub value: String,
}

/// Additional untyped data that can be present on most glTF types at the material level.
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-extras).
#[derive(Clone, Debug, Reflect, Default, Component)]
#[reflect(Component, Clone, Default, Debug)]
pub struct GltfMaterialExtras {
    /// Content of the extra data.
    pub value: String,
}

/// The material name of a glTF primitive.
///
/// See [the relevant glTF specification section](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-material).
#[derive(Clone, Debug, Reflect, Default, Component)]
#[reflect(Component, Clone)]
pub struct GltfMaterialName(pub String);
