// recolors materials with the extension 'myglb' to white

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_gltf_trait::*;

#[derive(Reflect,Component)]
#[reflect(Component)]
struct IsWhite;

#[derive(Reflect,Clone,Default)]
struct WhiteGltf;
impl GltfTrait for WhiteGltf {
    const EXTENSIONS: &'static [&'static str] = &["myglb"];
    type Material = StandardMaterial;        
    fn convert_material (mut convert:GltfTraitMaterial) -> Self::Material {
        convert.material.base_color = Color::WHITE;
        convert.material.base_color_texture = None;

        if let Some(extras) =  convert.raw.extras() {
            let map: HashMap<String,serde_json::Value> = serde_json::from_str(extras.get()).unwrap();
            if map.contains_key("is_green"){
                convert.material.base_color = Color::linear_rgb(0.,1.,0.);
            }
        }

        convert.material
    }
    fn on_app (app:&mut App) {
        app.register_type::<IsWhite>();
    }
    fn on_skinned_mesh (edit:GltfTraitEntity){
        edit.entity.insert(IsWhite);
    }
    fn on_mesh (edit:GltfTraitEntity){
        edit.entity.insert(IsWhite);
    }
}

fn main(){
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        GltfPlugin::<WhiteGltf>::default()
    ))
    .add_systems(Startup,startup);
    app.run();
}

fn startup (
    mut commands: Commands,
    assets:Res<AssetServer>,
    mut light:ResMut<AmbientLight>
){
    light.brightness = 5_000.;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.,0.,12.),
    ));
    commands.spawn((
        Transform::from_xyz(-1.2,0.,0.),
        SceneRoot(assets.load("red_box.glb#Scene0")),
    ));
    commands.spawn((
        Transform::from_xyz(1.2,0.,0.),
        SceneRoot(assets.load("red_box.myglb#Scene0").into()),
    ));
}
