use bevy::{
    math::Vec2,
    prelude::{App as BevyApp, AssetServer, Commands, Res, ResMut},
    window::{WindowDescriptor, Windows},
    PipelinedDefaultPlugins,
};
use bevy_kayak_ui::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_components::Window;
use kayak_core::Index;
use kayak_ui::components::App;
use kayak_ui::core::{rsx, widget};

#[widget]
fn TestState() {
    rsx! {
        <>
            <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Window 1".to_string()}>
                {}
            </Window>
            <Window position={(550.0, 50.0)} size={(200.0, 200.0)} title={"Window 2".to_string()}>
                {}
            </Window>
        </>
    }
}

fn startup(
    mut commands: Commands,
    windows: Res<Windows>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(asset_server.load("roboto.kayak_font"));

    let window_size = if let Some(window) = windows.get_primary() {
        Vec2::new(window.width(), window.height())
    } else {
        panic!("Couldn't find primary window!");
    };

    let context = BevyContext::new(window_size.x, window_size.y, |styles, context| {
        // Hack to trick the proc macro for right now..
        let parent_id: Option<Index> = None;
        rsx! {
            <App styles={Some(styles.clone())}>
                <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Window 1".to_string()}>
                    {}
                </Window>
                <Window position={(800.0, 50.0)} size={(200.0, 200.0)} title={"Window 2".to_string()}>
                    {}
                </Window>
            </App>
        }
    });

    commands.insert_resource(context);
}

fn main() {
    BevyApp::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("UI Example"),
            ..Default::default()
        })
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(BevyKayakUIPlugin)
        .add_startup_system(startup)
        .run();
}
