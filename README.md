# Editable Bevy glTF

This is a fork of [bevy](https://github.com/bevyengine/bevy) `/crates/bevy_gltf`, that doesn't change any functionalities, but provides several possibilities to customize the conversion between gltf and bevy interns to `edit` certain parts `on load`.

# Trait Features
- set the extensions `default: ["gltf", "glb"]`
- Lights:
    - edit their `SpotLight`, `PointLight` or `DirectionalLight` components
    - edit their `EntityWorldMut` (similar to `EntityCommands`)
    - edit the `Transform` of their parent
    - edit the `EntityWorldMut` of their parent