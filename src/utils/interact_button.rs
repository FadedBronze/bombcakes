use bevy::prelude::*;

pub trait HoverButton {
    fn on_click(commands: &mut Commands);
    fn get_interaction_colors() -> InteractionColors {
        InteractionColors {
            hover_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
            normal_color: Color::rgba(0.0, 0.0, 0.0, 0.8),
        }
    }
}

pub struct InteractionColors {
    pub hover_color: Color,
    pub normal_color: Color,
}

pub fn interact_system<T: HoverButton + bevy::prelude::Component>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<T>)>,
    mut commands: Commands,
) {
    let Ok((interaction, mut background_color)) = button_query.get_single_mut() else {
      return;
    };

    let colors = T::get_interaction_colors();

    match *interaction {
        Interaction::Clicked => {
            T::on_click(&mut commands);
        }
        Interaction::Hovered => *background_color = BackgroundColor(colors.hover_color),
        Interaction::None => *background_color = BackgroundColor(colors.normal_color),
    }
}
