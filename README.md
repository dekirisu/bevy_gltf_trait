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

# Trait Features
- set the extensions `default: &["gltf", "glb"]`
- Material:
    - change the `Material` used
    - or just edit the `StandardMaterial`s
- Meshes:
    - edit any `Mesh`
    - edit their `EntityWorldMut` (similar to `EntityCommands`)
    - edit the `Transform` and `EntityWorldMut` of their parent
- Lights:
    - edit their `SpotLight`, `PointLight` or `DirectionalLight` components
    - edit their `EntityWorldMut` 
    - edit the `Transform` and `EntityWorldMut` of their parent
- edit the `App`

# Notes
- If you want to insert components through the trait and they are foreign to bevy_gltf: 
    - Make sure to use `on_app` to `.register_type()` them
- the provided gltf structs make it possible to react to custom gltf properties

# Versions
| Bevy | This |
| ---- | ---- |
| 0.15 | 0.2  |
| 0.14 | 0.1  |

# Example
The original way of adding the plugin changes to:
```rust
fn main(){
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        GltfPlugin::<()>::default(),
        // ...
    ));
    app.run();
}
```
..and can be modified with the trait to either **replace** or **extend** (using different extensions) scene imports.
```rust 
#[derive(Reflect,Default)]
struct WhiteGltf;
impl GltfTrait for WhiteGltf {
    const EXTENSIONS: &'static [&'static str] = &["myglb"];
    type Material = StandardMaterial;        
    fn convert_material (mut convert:GltfTraitMaterial) -> Self::Material {
        convert.material.base_color = Color::WHITE;
        convert.material.base_color_texture = None;
        convert.material
    }
}

fn main(){
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        GltfPlugin::<WhiteGltf>::default()
    ));
    app.run();
}
```

## Bevy support table
| bevy | bevy_gltf_trait |
|------|-----------|
| 0.14 | 0.1      |
