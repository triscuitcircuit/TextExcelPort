use iced::{button, container, Background, Color, Vector};

// Colors
pub const COLOR_YELLOW: Color = Color::from_rgb(1., 0.812, 0.);
pub const COLOR_LIGHT_YELLOW: Color = Color::from_rgb(0.996, 0.952, 0.827);
pub const COLOR_LIGHTER_YELLOW: Color = Color::from_rgb(1., 0.992, 0.933);
pub const COLOR_GREEN: Color = Color::from_rgb(0., 0.592, 0.518);
pub const COLOR_LIGHT_GREEN: Color = Color::from_rgb(0.665, 0.941, 0.598);
pub const COLOR_LIGHTER_GREEN: Color = Color::from_rgb(0.85, 1., 0.85);
pub const COLOR_RED: Color = Color::from_rgb(1., 0., 0.);
pub const COLOR_DARK_RED: Color = Color::from_rgb(0.6, 0., 0.);
pub const COLOR_BROWN: Color = Color::from_rgb(0.38, 0.094, 0.129);
pub const DARK_GREY: Color = Color{r: 0.25, g: 0.25, b: 0.25, a: 1.0};
pub const LIGHT_GREY: Color = Color{r: 0.75, g: 0.75, b: 0.75, a: 1.0};
pub const BLACK: Color = Color{r: 0.00, g: 0.00, b: 0.00, a: 1.00};

pub enum Button{
    ToolBar,
    SheetList{selected: bool},
    Basic,
    Blue
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
            Button::SheetList {selected}=> button::Style{
              border_color: DARK_GREY,
                border_width: 1 as f32,
                border_radius: 4 as f32,
                background: if *selected {Some(Background::Color(LIGHT_GREY))} else {None},
                ..button::Style::default()
            },
            Button::Basic => button::Style{
                border_color: BLACK,
                border_width: 1 as f32,
                border_radius: 4 as f32,
                background: None,
                ..button::Style::default()
            },
            Button::Blue=> button::Style{
                background: Some(Background::Color(
                    Color::from_rgb(0.2, 0.2, 0.7),
                )),
                border_radius: 10.0,
                text_color:Color::WHITE,
                ..button::Style::default()
            }
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
    Basic,
    Selection
}
impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Container::Selection => container::Style{
                border_color: COLOR_GREEN,
                border_radius: 4 as f32,
                border_width: 1 as f32,
                background: None,
                ..container::Style::default()
            },
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