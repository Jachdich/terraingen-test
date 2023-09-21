use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use leafwing_input_manager::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    use Action::*;
    use KeyCode::{ShiftLeft, Space, A, D, S, W};
    commands
        .spawn(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (W, Forward),
                (S, Backward),
                (A, Left),
                (D, Right),
                (Space, Jump),
                (ShiftLeft, Crouch),
            ]),
        })
        .insert(Player);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 100,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

const UP_SPEED: f32 = 3.0;
const SPEED: f32 = 4.0;

fn move_and_look(
    action_query: Query<&ActionState<Action>, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let action_state = action_query.single();
    let mut camera_state = camera_query.single_mut();

    let mut forward = camera_state.forward();
    forward.y = 0.0;
    let mut left = camera_state.left();
    left.y = 0.0;
    
    if action_state.pressed(Action::Jump) {
        camera_state.translation += Vec3::Y * time.delta_seconds() * UP_SPEED;
    }
    if action_state.pressed(Action::Crouch) {
        camera_state.translation -= Vec3::Y * time.delta_seconds() * UP_SPEED;
    }

    if action_state.pressed(Action::Forward) {
        camera_state.translation += forward * time.delta_seconds() * SPEED;
    }
    if action_state.pressed(Action::Backward) {
        camera_state.translation -= forward * time.delta_seconds() * SPEED;
    }
    if action_state.pressed(Action::Left) {
        camera_state.translation += left * time.delta_seconds() * SPEED;
    }
    if action_state.pressed(Action::Right) {
        camera_state.translation -= left * time.delta_seconds() * SPEED;
    }

}

fn look(mut motion_ev: EventReader<MouseMotion>) {
    for ev in motion_ev.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (spawn_camera, spawn_scene, spawn_player))
        .add_systems(Update, (move_and_look, look, cursor_grab_system))
        .run();
}
