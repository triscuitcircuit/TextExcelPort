use iced::{button, container, Background, Color, Vector};

const DARK_GREY: Color = Color{r: 0.25, g: 0.25, b: 0.25, a: 1.0};
const LIGHT_GREY: Color = Color{r: 0.75, g: 0.75, b: 0.75, a: 1.0};
const BLACK: Color = Color{r: 0.00, g: 0.00, b: 0.00, a: 0.00};

pub enum Button{
    ToolBar,
    Basic,
}

impl button::StyleSheet for Button{
    fn active(&self) -> button::Style {
        match self{
            Button::ToolBar => button::Style{
                border_color: Color::TRANSPARENT,
                text_color: BLACK,
                background: None,
                ..button::Style::default()
            },
            Button::Basic => button::Style{
                border_color: BLACK,
                border_width: 1 as f32,
                border_radius: 4 as f32,
                background: None,
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style{
            background: Some(Background::Color(LIGHT_GREY)),
            ..active
        }
    }
}
pub enum Container{
    Basic
}
impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Container::Basic => container::Style {
                border_color: DARK_GREY,
                border_radius: 4 as f32,
                border_width: 1 as f32,
                background: None,
                ..container::Style::default()
            }
        }
    }
}