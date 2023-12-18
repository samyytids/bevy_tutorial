use crate::Player;
use crate::Money;
use bevy::prelude::*;

// To create a plugin I just need a unit struct that has the Plugin trait 
// implemented. Plugins can include other plugins so if one plugin requires
// another plugin that's all good in the hood. 
pub struct PigPlugin;

impl Plugin for PigPlugin {
    // This function effectively handles what happens to the app when you add
    // it via .add_plugins(), this massively reduces the amount of boilerplate
    // needed. 
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pig_parent)
            .add_systems(Update, (spawn_pig, pig_lifetime))
            .register_type::<Pig>();
    }
}

// Let's add a component for our pig, these pigs will age and then die giving
// the player some money. 
// The timer is a tool that does what you think it does. 
#[derive(Component, Default, Reflect)]
// reflect allows for editing your custom values. To make this usable we need
// to register the type to our app.  
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
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
    parent: Query<Entity, With<PigParent>>,
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
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent £10 on a pig, you now have: £{:?}", money.0);

        let texture: Handle<Image> = asset_server.load("pig.png");

        // This spawns a pig text at the players location, this is the 
        // implementation if we are not using a parent. 
        /*
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
        */

        // This spawns a pig if we are using a parent to spawn child pigs. 
        // This basically says .with_children(|child builder|) { how to build }
        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Pig {
                    lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                },
                Name::new("Pig"),
            ));
        });
    }
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
    parent: Query<Entity, With<PigParent>>,
) {
    let parent = parent.single();
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 20.0;
            // commands.entity returns us a data type that allows us to make a
            // variety of changes to the entity that we pass it. We can add 
            // components to them, fetch their ids and various other 
            // functionalities. 
            // In this case we are simply despawning them. 
            // This is how we do that without a parent. 
            /* 
            commands.entity(pig_entity).despawn();
            */

            // This is how we do that with a parent. First we get the entity
            // for the parent, we then remove the child by passing a slice of
            // the child entity. 
            commands.entity(parent).remove_children(&[pig_entity]);
            // We then despawn the pig entity in the same way as we do above,
            // if I don't remove child the list will get larger and larger 
            // creating a sort of memory leak, the parent won't ever try to 
            // actually access the child if you don't delete it. And if we 
            // iterate over the list we will be iterating over dead entities. 
            commands.entity(pig_entity).despawn();

            // This logs to the console. 
            info!("Pig sold for £20! Current money: £{:?}", money.0);
        }
    }
}

// Bevy allows for parent child hierarchies which means we can move things
// around as groups based on their parent or move them relative to their
// parent depending on if we use Transform or GlobalTransform. 

#[derive(Component)]
pub struct PigParent;

fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn(
        // This propogates movement stuff from the parent to the child
        // even when using empty parent structs. 
        (SpatialBundle::default(), PigParent, Name::new("Pig parent"),)
    );
}