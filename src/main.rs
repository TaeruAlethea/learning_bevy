use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_map));
    app.add_systems(
        Update,
        (
            player_look,
            focus_events,
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
        ),
    );
    app.add_observer(apply_grab);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Player));
}

fn spawn_map(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLight::default());
    let ball_mesh = mesh_assets.add(Sphere::new(1.));
    for h in 0..16 {
        let color = Color::hsl((h as f32 / 16.) * 360., 1., 0.5);
        let ball_material = material_assets.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });

        commands.spawn((
            Transform::from_translation(Vec3::new((-8. + h as f32) * 2., 0., -50.0)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
    }
}

#[derive(Component)]
struct Player;

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::{CursorOptions, PrimaryWindow};
fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    let sensitivity = 100. / window.width().min(window.height());
    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch = pitch.clamp(-1.57, 1.57);
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
}

#[derive(Event, Deref)]
struct IsCursorGrabedEvent(bool);

fn apply_grab(
    grab: On<IsCursorGrabedEvent>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    use bevy::window::CursorGrabMode;
    if **grab {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

use bevy::window::WindowFocused;
fn focus_events(mut events: MessageReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(IsCursorGrabedEvent(event.focused));
    }
}
fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(IsCursorGrabedEvent(window.focused));
}
