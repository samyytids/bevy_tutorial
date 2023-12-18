// Importing the main parts of the bevy engine
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions};
mod pigs;
mod ui;
use pigs::*;
use ui::GameUi;
// Creating systems are functions that do the actual running of the game but
// they require a specific set of types as inputes, these can be commands.

// Commands are how you interact with the game and can be used to spawn things
// they are called periodically throughout the frame. They don't take effect
// instantly.

// First we need to spawn a camera, because without a camera we will always
// have a black screen. We can do this using the spawn command which takes
// a bundle of components to spawn a new entity.

// Entity's are the things that make up the game world and are simply made of
// an id and no inate data. 

// The data is held by components. 

// Entity example: The camera.
// The camera holds a series of components that contain all of the data that
// controls how the camera works. 

/*
pub struct Camera2Bundle {
    pub camera: Camera, 
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
    pub tonemapping: Tonemapping,
    pub deband_dither: DebandDither,
}
*/

// This creates a system that we intend to run on start-up that spawns a
// camera with default values as well as spawning a sprite to show that the
// game is running. 
// AssetServer is a resource these are single instance services so things that
// we don't need more than one of like our asset loader and other global data.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::PURPLE),
        },
        // ..default effectively says hey I don't care about the rest of the 
        // parameters, they can all use the defaults. 
        ..default()
    });

    // This loads from the default location of asset/filename.png. This is just
    // a cheap reference to the image data, it doesn't load the data itself so 
    // free and easy copy and move. 

    // By default bevy uses some filtering on assets to make em smooth, with 
    // pixel art that isn't desirable. We can address this by changing some of
    // the default plugins.
    let texture = asset_server.load("Sprite-0001.png");

    // We have now updated this so that it only takes the texture we have made
    // and now addedd the Player component to it which now means we can set the
    // speed that our plays at here rather than repeatedly in our movement 
    // system. The other added benefit is now in our movement code we don't 
    // need to call for the more generic Sprite component. As such we now know
    // that the only thing that will appear in our character_movement system's
    // query will be our Player entity. 
    // Not that encapsulating these traits in a tuple makes them part of one 
    // bundle. 
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0},
        Name::new("Player"),
    ));
}

// Let's make it so that we can move our character around shall we?
// We currently only have 1 object with the trait sprite so this query is 
// adequate. If we had more than one sprite then we could end up having 
// problems. What this basically does is say, hey anything with the traits
// transform and &Sprite need to have this applied to them. We also need
// to consider the player's input which comes from the input resource with the
// input type being KeyCodes. We also make use of time to 
fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // Here we are looping over all of the entities that match our query and 
    // applying the function below to them. 
    for (mut transform, player) in &mut characters {
        let movement_speed = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_speed;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_speed;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_speed;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_speed;
        }
    }
}

// Making your own component:

// The only restrictions for Components is that they need to be Send and Sync
// other than that they can be structs, enums you name it any valid rust data
// structure is fine. 

// Here we are making the player component which contains information about
// how fast the player moves and at the moment does nothing else.
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    // This allows us to set a minimum valuie in the debug menu, this way we
    // cannot have negative speed. 
    #[inspector(min=0.0)]
    pub speed: f32,
}


// Making your own resource: 

// Remember the main use for resources is for things that we think we will only
// need one of. In this case we are going to use a money counter, why would I 
// need multiple things to keep track of how much money I have? It's a global
// thing that I only need 1 value for so only one thing needs to keep track of
// it. 

// In this case we are using a simple tuple struct since the struct only needs
// to keep track of one thing and has nothing else that it needs to do. 
// You can implement the Default trait in order to allow for default values to 
// be created using an initializer. 
#[derive(Resource)]
pub struct Money(pub f32);

// This is an implementation of the Default trait, this allows us to add this
// resource to our app using the init_resouce<ResouceName> syntax instead of
// the insert_resource(ResourceName { values: value }) syntax. 
impl Default for Money {
    fn default() -> Self {
        Money(100.0)
    }
}

// Let's use a community plugin, this is a great debugging plugin. 

fn main() {
    // add_systems requires a schedule as well as the system, in simple
    // terms we effectively say hey I want you do do this and this is when I 
    // would like you to do it. 

    // Startup only executes once upon startup.
    // Update executes on every frame. 
    App::new()
        .add_plugins(
            DefaultPlugins
                // This is us changing some of the defaults so that we can use
                // non-blurry sprites as well as changing the window name as 
                // well as changing the resolution and making the window non-
                // resizable.
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Test game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        // All the pig related code has now been moved to a separate file this
        // means that I no longer need to add each system separately that is
        // now all handled within that file. 
        .add_plugins((PigPlugin, GameUi))
        // This plugin allows for a really spicy debug menu, but it has gross
        // names, in order to fix that you can add the Name trait to your spawn
        // bundles. 
        .add_plugins(
            WorldInspectorPlugin::default()
                // This says whether a plugin shouldbe ran depending on a 
                // condition. This condition says that this should only run if
                // the escape key is pressed. 
                .run_if(
                    input_toggle_active(true, KeyCode::Escape)),
        )
        .init_resource::<Money>()
        .add_systems(Startup, setup)
        /*
        Systems with the same scheduler can be added in one step by providing
        them within a tuple. 
        .add_systems(Update, character_movement)
        .add_systems(Update, spawn_pig)
        .add_systems(Update, pig_lifetime)
        */
        .add_systems(Update, character_movement)
        .run();
}
