use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ball::Ball, config::CONFIG};

#[derive(Component)]
pub struct Selection {
    pub selected: bool,
}

pub fn select(commands: &mut Commands, entity: Entity) {
    eprintln!("selected entity!!!");
    commands.entity(entity).insert(Selection { selected: true });
}

pub fn de_select(commands: &mut Commands, entity: Entity) {
    eprintln!("de selected entity!!!");
    commands
        .entity(entity)
        .insert(Selection { selected: false });
}

#[derive(Component)]
pub struct Highlight;

pub fn highlight_selected(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    balls_to_highlight_with_children_q: Query<(&mut Selection, Entity, &Children, With<Ball>)>,
    balls_to_highlight_without_children_q: Query<(&mut Selection, Entity, Without<Children>, With<Ball>)>,
    highlight_q: Query<&Highlight>,
) {
    for (selection, entity, _, _) in &balls_to_highlight_without_children_q {
        if selection.selected {
            let highlight = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(CONFIG.ball_radius + 5.0).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::GOLD)),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                    ..default()
                })
                .insert(Highlight)
                .id();
            commands.entity(entity).add_child(highlight);
        }
    }
    for (selection, entity, children, _) in &balls_to_highlight_with_children_q {
        if let Some(child) = children
            .iter()
            .find(|child| highlight_q.get(**child).is_ok())
        {
            if !selection.selected {
                commands.entity(entity).remove_children(&[*child]);
            }
        }
    }
}
