use ecs::World;
use ecs::eql::resolver;
use ecs_utils::*;
use env_logger;
use kiss3d::{
    event::{Key, WindowEvent},
    prelude::*,
};
use visual::Visual;
use wisk::prelude::*;

const DT: f64 = 0.01;

fn add_entity(world: &mut World, position: Position3D) {
    let entity = world.spawn();
    world.add_component(entity.id, position);

    world.add_component(
        entity.id,
        Velocity3D(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    );

    world.add_component(
        entity.id,
        Rotation3D(Quaternion::from_axis(
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        )),
    );
    world.add_component(entity.id, Visulized {});
    world.add_component(entity.id, Cube3D { edge_size: 1.0 });
}

#[kiss3d::main]
async fn main() {
    env_logger::init();

    let mut visual = Visual::new();
    let mut window = Window::new("AUS").await;
    let mut camera = OrbitCamera3d::default();
    let mut world = World::new();

    add_entity(
        &mut world,
        Position3D(Vector3 {
            x: 0.0,
            y: -2.0,
            z: 10.0,
        }),
    );

    add_entity(
        &mut world,
        Position3D(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        }),
    );

    let entities = resolver("(SELECT WITH Position3D)", &world);

    match entities {
        Ok(entities) => {
            for e in entities {
                println!("{:?}", e.id)
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let mut direction_y = 0.0;
    let mut direction_x = 0.0;
    let mut spawn = false;

    while window.render_3d(&mut visual.scene, &mut camera).await {
        for event in window.events().iter() {
            if let WindowEvent::Key(Key::K, Action::Press, _) = event.value {
                direction_y += 1.0;
            }
            if let WindowEvent::Key(Key::J, Action::Press, _) = event.value {
                direction_y -= 1.0;
            }
            if let WindowEvent::Key(Key::L, Action::Press, _) = event.value {
                direction_x -= 1.0;
            }
            if let WindowEvent::Key(Key::H, Action::Press, _) = event.value {
                direction_x += 1.0;
            }
            if let WindowEvent::Key(Key::Space, Action::Press, _) = event.value {
                spawn = true;
            }
        }
        visual.update(&mut world);
        moving_system(
            &mut world,
            &mut camera,
            direction_y,
            direction_x,
            &mut spawn,
        );
    }
}

fn moving_system(
    mut world: &mut World,
    camera: &mut OrbitCamera3d,
    direction_y: f64,
    direction_x: f64,
    spawn: &mut bool,
) {
    let entities = world.query("(SELECT WITH Position3D)").unwrap();

    let mut vec = Vec3::new(0.0, 0.0, 1.0);
    for e in &entities {
        if e.id != 1 {
            continue;
        }
        let position = &mut world.get_mut_component::<Position3D>(e.id).unwrap();
        vec = Vec3 {
            x: position.0.x as f32,
            y: position.0.y as f32,
            z: position.0.z as f32,
        };

        camera.look_at(camera.eye(), vec);

        position.0 += DT
            * Velocity3D {
                0: Vector3 {
                    x: direction_x,
                    y: direction_y,
                    z: 0.0,
                },
            }
            .0;
    }
    if *spawn {
        add_entity(
            &mut world,
            Position3D(Vector3 {
                x: vec.x as f64,
                y: vec.y as f64,
                z: vec.z as f64,
            }),
        );
        *spawn = !*spawn;
    }
}
