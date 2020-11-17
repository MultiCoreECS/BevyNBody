use bevy::prelude::*;
use bevy::app::AppExit;
use rand::Rng;

const G: f32 = 0.0000000000667430;

fn main() {
    let matches = clap::App::new("bevy_balls")
        .version("1.0")
        .author("James Bell")
        .about("N body simulation experiment for class")
        .arg(clap::Arg::with_name("room_size")
            .short("r")
            .long("room_size")
            .help("Sets the room size, number of balls is equal to room_size^2")
            .takes_value(true))
        .arg(clap::Arg::with_name("max_iter")
            .short("m")
            .long("max_iter")
            .help("How many iterations to run")
            .takes_value(true))
        .get_matches();

    let room_size = matches.value_of("room_size").unwrap_or("10.0").parse::<f32>().unwrap_or(10.0);
    let max_iter = matches.value_of("max_iter").unwrap_or("100000").parse::<i32>().unwrap_or(100000);

    App::build()
        .add_resource(Room{x: room_size/2.0, y: room_size/2.0})
        .add_resource(Counter{current: 0, max: max_iter})
        .add_plugins(MinimalPlugins)
        .add_startup_system(start.system())
        .add_resource(Time::default())
        .add_system(update_positions.system())
        .add_system(update_velocities.system())
        .add_system(count_then_exit.system())
        .run();
}

struct Position{
    x: f32,
    y: f32
}

struct Velocity{
    x: f32,
    y: f32
}

struct Room{
    x: f32,
    y: f32
}

fn start(mut commands: Commands, room: Res<Room>){
    let mut rng = rand::thread_rng();
    for i in 0..(((room.x * 2.0) * (room.y * 2.0))as isize){
        commands.spawn((
            Position{
                x: rng.gen_range(-room.x, room.x),
                y: rng.gen_range(-room.y, room.y)
            },
            Velocity{
                x: 0.0,
                y: 0.0
            }
        ));
    }
}

fn update_positions(time: Res<Time>, mut query: Query<(&Velocity, &mut Position)>){
    for (vel, mut pos) in query.iter_mut(){
        pos.x += vel.x * time.delta_seconds;
        pos.y += vel.y * time.delta_seconds;
    }
}

fn update_velocities(time: Res<Time>, mut query: Query<(&mut Velocity, &Position)>, pos_query: Query<&Position>){
    for (mut vel, pos) in query.iter_mut(){
        for (pos_other) in pos_query.iter(){
            let direction_x = pos.x - pos_other.x;
            let direction_y = pos.y - pos_other.y;
            let direction_mag_squared = direction_x.powi(2) + direction_y.powi(2);
            let direction_mag = direction_mag_squared.sqrt();

            let force_mag = G /(direction_mag_squared);
            vel.x += force_mag * direction_x / direction_mag * time.delta_seconds;
            vel.y += force_mag * direction_y / direction_mag * time.delta_seconds;
        }
    }

    println!("{}", time.delta_seconds);
}

struct Counter{
    current: i32,
    max: i32,
}

fn count_then_exit(mut exit: ResMut<Events<AppExit>>, mut counter: ResMut<Counter>){
    if counter.current < counter.max{
        counter.current += 1;
    }
    else{
        exit.send(AppExit{});
    }
}