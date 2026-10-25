<h1 align="center">Bevy glTF Trait</h1>
<p align="center">
    <a href="https://github.com/dekirisu/bevy_gltf_trait" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu-ee6677">
    </a>
    <a href="https://crates.io/crates/bevy_gltf_trait" style="position:relative">
        <img src="https://img.shields.io/crates/v/bevy_gltf_trait">
    </a>
</p>

This is a fork of [bevy](https://github.com/bevyengine/bevy) `/crates/bevy_gltf`, that doesn't change any features, but enables modifications `on load` through a simple trait. e.g.:
- make the gltf-loader use a custom material on entities
- make it insert components based on custom gltf properties
- or more general: modify how to `gltf -> bevy`

# Usage

```rust 
fn main(){
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        // Use your modified Gltf-Loader
        GltfPlugin::<BlueGltf>::default()
    ));
    app.run();
}


#[derive(Reflect,Clone,Default)]
struct BlueGltf;

impl GltfTrait for BlueGltf {

    // Set Extensions for this custom loader
    const EXTENSIONS: &'static [&'static str] = &["gltf", "glb"];
    // Should morphs be loaded?
    const ENABLE_MORPHS: bool = true;

    // App access
    fn on_app (_app:&mut App){}

    // How to attach a material to an Entity
    fn attach_material(edit:GltfTraitMaterialAttach<Self::Material>) {
        edit.entity.insert(MeshMaterial3d(edit.handle));
    }

    // What Material should the Entities use?
    type Material = StandardMaterial;        
    // How to translate from gltf::Material and bevy::StandardMaterial to your choice?
    // e.g.: all base-colors shall be blue!
    fn convert_material (convert:GltfTraitMaterial) -> Self::Material {
        let gltf = convert.material;
        StandardMaterial{
            base_color: Color::srgb(0.,0.,1.),
            base_color_texture: gltf.base_color_texture,
            perceptual_roughness: gltf.perceptual_roughness,
            metallic: gltf.metallic,
            emissive: gltf.emissive,
            ..default()
        }
    }

    // Edit the entity of a mesh
    fn on_mesh (_edit:GltfTraitEntity){}
    // Edit meshes
    fn edit_mesh (_edit:GltfTraitMesh){}        

    // Edit the entity or Transform of any light
    fn on_light_parent (_edit:GltfTraitParent){}

    // Edit the entity or component of a DirectionalLight
    fn edit_directional_light (_edit:GltfTraitLight<DirectionalLight>){}
    // Edit the entity or component of a PointLight
    fn edit_point_light (_edit:GltfTraitLight<PointLight>){}
    // Edit the entity or component of a SpotLight
    fn edit_spot_light (_edit:GltfTraitLight<SpotLight>){}        

    // Edit the entity or Transform of a set of meshes 
    fn on_mesh_parent (_edit:GltfTraitParent){}      
    // Edit the entity or Transform of a set of skinned meshes 
    fn on_skinned_mesh_parent (_edit:GltfTraitParent){}
    // Edit the entity of a skinned mesh 
    fn on_skinned_mesh (_edit:GltfTraitEntity){}

}
```

# Versions
| Bevy | This |
| ---- | ---- |
| 0.20 | 0 8 |
| 0.19 | ❌ |
| 0.18 | 0.6 |
| 0.17 | 0.5 |
| 0.16 | 0.4 |
| 0.15 | 0.2 |
| 0.14 | 0.1 |


