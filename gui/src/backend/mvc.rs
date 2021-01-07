use iced::{
    button, executor, keyboard, pane_grid, scrollable, Align, Application,
    Button, Column, Command, Container, Element, HorizontalAlignment, Length,
    PaneGrid, Scrollable, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};
use std::collections::HashMap;
use crate::backend::messages::Message;
use crate::backend::{styles, toolbar};

pub struct Textexcel{
    toolbar: toolbar::ToolBar
}
impl Application for Textexcel{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self{
                toolbar: toolbar::ToolBar::default()
            },
            Command::none()
            )
    }

    fn title(&self) -> String {
        String::from("TextExcel-demo")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        unimplemented!()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(8)
            .spacing(8)
            .align_items(Align::Center)
            .push(self.toolbar.view())
            .into()
    }
}


fn handle_hotkey(key_code: keyboard::KeyCode)-> Option<Message>{
    unimplemented!()
}