<h1 align="center">Bevy glTF Trait</h1>
<p align="center">
    <a href="https://github.com/dekirisu/bevy_gltf_trait" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu-ee6677">
    </a>
    <a href="https://crates.io/crates/bevy_gltf_trait" style="position:relative">
        <img src="https://img.shields.io/crates/v/bevy_gltf_trait">
    </a>
</p>

This is a fork of [bevy](https://github.com/bevyengine/bevy) `/crates/bevy_gltf`, that doesn't change any functionalities, but provides several possibilities to `customize` the conversion between gltf and bevy interns `on load` using the trait `GltfTrait`. 

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

// Define your modified Gltf-Loader
#[derive(Reflect,Clone,Default)]
struct BlueGltf;

impl GltfTrait for BlueGltf {

    // Set Extensions for this custom loader
    const EXTENSIONS: &'static [&'static str] = &["gltf", "glb"];
    // Should morphs be loaded?
    const ENABLE_MORPHS: bool = true;

    // App access
    fn on_app (_app:&mut App){}

    // What Material should the Entities use?
    type Material = StandardMaterial;        
    // How to translate from gltf::Material and bevy::StandardMaterial to your choice?
    // e.g.: all base-colors shall be blue!
    fn convert_material (mut convert:GltfTraitMaterial) -> Self::Material {
        convert.material.base_color = Color::srgb(0.,0.,1.);
        convert.material.base_color_texture = None;
        convert.material
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

# Notes
- If you want to insert components through the trait and they are foreign to bevy_gltf: 
    - Make sure to use `on_app` to `.register_type()` them
- the provided gltf structs make it possible to react to custom gltf properties

# Original Loader
The original way of adding the plugin changes to:
```rust
fn main(){
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        GltfPlugin::<()>::default(),
    ));
    app.run();
}
```

# Versions
| Bevy | This |
| ---- | ---- |
| 0.17 | 0.4  |
| 0.16 | 0.3  |
| 0.15 | 0.2  |
| 0.14 | 0.1  |


