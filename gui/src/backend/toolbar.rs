use iced::{button, Button, Container, Row, Text };
use iced_native::{Align, Length};
use crate::backend::styles;
use crate::backend::messages::Message;


#[derive(Debug,Default)]
pub struct ToolBar{
    file_button: button::State,
    edit_button: button::State,
    macro_button: button::State
}
impl ToolBar{
    pub fn view(&mut self) -> Container<Message>{
        Container::new(
            Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(24)
                .push(
                    Button::new(&mut self.file_button,Text::new("File"))
                        .on_press(Message::ViewFileScreen)
                        .style(styles::Button::ToolBar),
                )
                .push(
                  Button::new(&mut self.edit_button, Text::new("Edit"))
                      .on_press(Message::ViewEditScreen)
                      .style(styles::Button::ToolBar)
                )
                .push(
                    Button::new(&mut self.macro_button,Text::new("Macro"))
                        .style(styles::Button::ToolBar)
                        .on_press(Message::ViewMacroScreen)
                ),

        )
            .style(styles::Container::Basic)
            .align_y(Align::Start)
            .width(Length::Fill)
            .padding(4)
    }
}