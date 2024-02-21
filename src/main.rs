use bevy::pbr::ScreenSpaceAmbientOcclusionTextures;
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::ColorTargetState;
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
struct Player {
    yaw: f32,
    pitch: f32,
}

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
        .insert(Player { yaw: 0.0, pitch: 0.0 });
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
    // let map = [0, 1, 6, 3, 1, 6, 4, 7, 3, 4, 4, 2, 1, 8, 7, 6, 7, 7, 3, 3, 1, 5, 3, 4, 7];
    let map = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let width = 5;
    let height = 5;
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleStrip);
    let mut vertices = Vec::<[f32; 3]>::new();
    // let mut colours = Vec::<[f32; 3]>::new();
    let mut indices = Vec::<u32>::new();

    for y in 0..(height - 1) {
        for x in 0..(width) {
            vertices.push([x as f32, y as f32, map[y * width + x] as f32]);
            vertices.push([x as f32, y as f32 + 1.0, map[(y + 1) * width + x] as f32]);
            // vertices.push([x as f32, y as f32 + 1.0, map[(y+1) * width + x] as f32]);
            indices.push(((y*width+x)*2+0) as u32);
            indices.push(((y*width+x)*2+1) as u32);
            // indices.push(((y*width+x)*3+2) as u32);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(vertices));
    // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float32x3(colours));
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial { base_color: Color::rgb(0.3, 0.5, 0.3), double_sided: true, ..default()}),
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

fn look(mut motion_ev: EventReader<MouseMotion>, mut camera_query: Query<&mut Transform, With<Camera3d>>, mut player_query: Query<&mut Player>) {
    let mut camera_state = camera_query.single_mut();
    let mut player = player_query.single_mut();
    const SENSITIVITY: f32 = 0.001;
    for ev in motion_ev.iter() {
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        player.yaw -= ev.delta.x * SENSITIVITY;
        player.pitch -= ev.delta.y * SENSITIVITY;
    }
    player.pitch = player.pitch.clamp(-3.141592/2.0, 3.141592/2.0); 
    camera_state.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw) * Quat::from_axis_angle(Vec3::X, player.pitch);
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
