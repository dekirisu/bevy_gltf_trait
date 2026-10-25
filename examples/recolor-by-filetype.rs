// recolors materials with the custom property 'is_green' to green

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_gltf_trait::*;
use bevy_color::palettes::css as Colors;

#[derive(Reflect,Clone,Default)]
struct WhiteOrGreenGltf;
impl GltfTrait for WhiteOrGreenGltf {
    const EXTENSIONS: &'static [&'static str] = &["myglb"];
    type Material = StandardMaterial;        
    fn attach_material(edit:GltfTraitMaterialAttach<Self::Material>) {
        edit.entity.insert(MeshMaterial3d(edit.handle));
    }
    fn convert_material(mut convert:GltfTraitMaterial) -> Self::Material {
        let mut is_green = false;
        convert.material.base_color_texture = None;
        if let Some(extras) =  convert.raw.extras() {
            let map: HashMap<String,serde_json::Value> = serde_json::from_str(extras.get()).unwrap();
            is_green |= map.contains_key("is_green");
        }
        StandardMaterial{
            base_color: match is_green {
                true => Colors::GREEN,
                false => Colors::WHITE,
            }.into(),
            ..default()
        }
    }
}

fn main(){
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        GltfPlugin::<WhiteOrGreenGltf>::default()
    ))
    .add_systems(Startup,startup);
    app.run();
}

fn startup(
    mut commands: Commands,
    assets:Res<AssetServer>,
    mut light:ResMut<GlobalAmbientLight>
){
    light.brightness = 5_000.;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.,0.,12.),
    ));
    commands.spawn((        
        Transform::from_xyz(-1.2,0.,0.),
        WorldAssetRoot(assets.load("red_box.glb#Scene0")),
    ));
    commands.spawn((        
        Transform::from_xyz(1.2,0.,0.),
        WorldAssetRoot(assets.load("red_box.myglb#Scene0"))
    ));
}
