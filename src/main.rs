// Importing the main parts of the bevy engine
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

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
#[derive(Component)]
pub struct Player {
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

// You can also add the "special" trait FromWorld which allows us to create 
// resources that have access to the entire bevy ECS world. This is moslty
// useful for things like rendering. 

// Let's add some actual "gameplay" shall we? This allows us to spawn pigs at
// the cost of 10 dollars every time we press the spacebar. 
fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    // Mutability needed since we are changing the amount of money that we have
    // otherwise we would be creating pigs for free. 
    mut money: ResMut<Money>,
    // We don't need a mutable player since all we are doing with the player is
    // saying hey, where are you? Oh there, so that's where the pig is going. 
    // This query unlike the one in movement is a filter query, the first one
    // says that I am wanting to get a transform and that transform needs to 
    // come with the Player component.  
    // Because of rust's borrow checker we need to be conservative with whether
    // something will need read/write access. Bevy tries to do things in 
    // parallel where they can, this means that we can end up with blocking
    // behaviour if something tries to get read access while something else
    // currently has write access. 
    player: Query<&Transform, With<Player>>,
) {
    // This forces the function to skip out on the rest of the function if we 
    // have already pressed the spacebar recently, think the double jump issue
    // we had when messing around with unity. 
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    // Single is an unrecoverable state, think unwrap. If it isn't a single 
    // instance then we will get a panic. If we want a recoverable state IE I 
    // know that there may be more than 1 thing in this query but I have a 
    // process that I can use to determine which one I actually want. I would
    // want to use get_single() instead. This returns a Result and provides 
    // information about whether the query beansed because there was no result
    // or if there were too many results. 
    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent £10 on a pig, you now have: £{:?}", money.0);

        let texture: Handle<Image> = asset_server.load("pig.png");

        // This spawns a pig text at the players location, not deref because 
        // player.single 
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            // This adds the pig component to the entity.
            Pig {
                // This pig has a timer in it that lasts for 2 seconds and 
                // executes a single time. IE it will hit 2.0 and that's it. 
                // Note timers do not manually tick, we need to keep track of 
                // that ourselves. Which we do later. (fn pig_lifetime)
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

// Let's add a component for our pig, these pigs will age and then die giving
// the player some money. 
// The timer is a tool that does what you think it does. 
#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

// This system is used to keep track of the pig's timer. 
fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    // Note that entity is special and is the only thing we have in the first
    // part of a query that doesn't need to be used as a reference. 
    mut pigs: Query<(Entity, &mut Pig)>,
    // Spawn pig and the pig_lifetime systems both mutably acces money, this 
    // means that we will have a block here. But, since these are both very 
    // small systems it is unlikely that this will cause issues. But, for large
    // systems that take a long time to resolve this could be an issue. 
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 20.0;
            // commands.entity returns us a data type that allows us to make a
            // variety of changes to the entity that we pass it. We can add 
            // components to them, fetch their ids and various other 
            // functionalities. 
            // In this case we are simply despawning them. 
            commands.entity(pig_entity).despawn();

            // This logs to the console. 
            info!("Pig sold for £20! Current money: £{:?}", money.0);
        }
    }
}

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
        .init_resource::<Money>()
        .add_systems(Startup, setup)
        /*
        Systems with the same scheduler can be added in one step by providing
        them within a tuple. 
        .add_systems(Update, character_movement)
        .add_systems(Update, spawn_pig)
        .add_systems(Update, pig_lifetime)
        */
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
        .run();
}
