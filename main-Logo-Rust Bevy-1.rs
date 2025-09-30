use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct DragState {
    last_position: Option<Vec2>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.init_resource::<DragState>();

    let points = vec![
(1.882716049382716,395.0),
(1.7129629629629628,305.0),
(2.2222222222222223,290.0),
(2.20679012345679,205.0),
(2.716049382716049,207.5),
(2.7932098765432096,115.0),
(3.317901234567901,145.0),
(3.4876543209876543,50.0),
(4.012345679012346,97.5),
(4.243827160493827,2.00),
(4.7067901234567895,72.5),
(5.061728395061728,12.5),
(5.432098765432098,7.05),
(5.895061728395062,2.00),
(6.111111111111111,100.0),
(6.62037037037037,60.0),
(6.805555555555555,137.5),
(7.330246913580247,117.5),
(7.345679012345679,207.5),
(7.901234567901234,202.5),
(7.901234567901234,295.0),
(8.441358024691358,302.5),
(8.333333333333332,387.5),
(8.796296296296296,422.5),
(8.549382716049383,49.0),
(8.996913580246913,542.5),
(8.68827160493827,617.5),
(9.135802469135802,672.5),
(8.672839506172838,727.5),
(9.043209876543209,795.0),
(8.580246913580247,840.0),
(8.780864197530864,917.5),
(8.287037037037036,945.0),
(8.410493827160494,1035.0),
(7.885802469135802,1047.5),
(7.932098765432098,1140.0),
(7.4074074074074066,1127.5),
(7.314814814814814,1220.0),
(6.805555555555555,1200.0),
(6.604938271604938,1272.5),
(6.126543209876543,1237.5),
(5.416666666666666,1252.5),
(4.7067901234567895,1260.0),
(3.580246913580247,1267.5),
(4.012345679012346,1230.0),
(2.839506172839506,1212.5),
(3.333333333333333,1190.0),
(2.824074074074074,1125.0),
(2.20679012345679,1127.5),
(2.2376543209876543,1045.0),
(1.6975308641975309,1030.0),
(1.8518518518518516,947.5),
(1.3271604938271604,915.0),
(1.5586419753086418,840.0),
(1.126543209876543,790.0),
(1.4351851851851851,727.5),
(1.0339506172839505,672.5),
(1.404320987654321,615.0),
(1.095679012345679,542.5),
(1.5586419753086418,490.0),
(1.3425925925925926,410.0),
(2.9320987654320985,345.0),
(4.506172839506172,202.5),
(5.061728395061728,275.0),
(5.617283950617284,202.5),
(7.6080246913580245,432.5),
(7.4074074074074066,547.5),
(7.993827160493827,615.0),
(7.978395061728395,702.5),
(7.731481481481481,710.0),
(7.484567901234567,785.0),
(7.021604938271604,777.5),
(6.5895061728395055,650.0),
(7.098765432098765,480.0),
(6.188271604938271,342.5),
(4.4907407407407405,497.5),
(5.817901234567901,527.5),
(4.459876543209877,572.5),
(4.4907407407407405,720.0),
(4.459876543209877,807.5),
(5.1080246913580245,807.5),
(5.138888888888888,950.0),
(2.716049382716049,957.5),
(2.947530864197531,987.5),
(3.657407407407407,982.5),
(3.82716049382716,1095.0),
(6.342592592592593,110.0),
(6.512345679012346,982.5),
(7.160493827160494,990.0),
(7.391975308641975,955.0),
(6.064814814814815,942.5),
(5.802469135802469,862.5),
(5.617283950617284,767.5),
(5.277777777777778,717.5),
(3.1790123456790123,807.5),
(3.16358024691358,495.0),
(2.700617283950617,497.5),
(2.7777777777777777,545.0),
(2.1604938271604937,610.0),
(2.2685185185185186,800.0)






    ];

    let scale = 10.0;
    let offset_x = -500.0;
    let offset_y = -500.0;

    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));

    for (i, &(x, y)) in points.iter().enumerate() {
        let pos_x = x * scale + offset_x;
        let pos_y = y * scale + offset_y;

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(3.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            ..default()
        });

        if let Some(&(next_x, next_y)) = points.get(i + 1) {
            let next_pos_x = next_x * scale + offset_x;
            let next_pos_y = next_y * scale + offset_y;
            
            let dir = Vec2::new(next_pos_x - pos_x, next_pos_y - pos_y);
            let length = dir.length();
            let angle = dir.angle_between(Vec2::X);
            
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(length, 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (pos_x + next_pos_x) / 2.0,
                    (pos_y + next_pos_y) / 2.0, 
                    0.0
                ).with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            });
        }
    }
}

fn camera_control(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut drag_state: ResMut<DragState>,
) {
    let window = windows.single();
    
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            drag_state.last_position = Some(position);
        }
    }

    if mouse.pressed(MouseButton::Left) {
        if let (Some(last_pos), Some(current_pos)) = (drag_state.last_position, window.cursor_position()) {
            let mut transform = camera.single_mut();
            let delta = current_pos - last_pos;
            
            transform.translation.x -= delta.x;
            transform.translation.y += delta.y;
            
            drag_state.last_position = Some(current_pos);
        }
    }

    if mouse.just_released(MouseButton::Left) {
        drag_state.last_position = None;
    }
}