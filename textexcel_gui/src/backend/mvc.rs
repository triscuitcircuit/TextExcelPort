use iced::{
    button, executor, keyboard, pane_grid, scrollable, Align, Application,
    Button, Column, Command, Container, Element, HorizontalAlignment, Length,
    PaneGrid, Scrollable, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};
use std::collections::HashMap;
use crate::backend::messages::Message;
use crate::backend::{styles, toolbar};
use crate::backend::macro_screen::MacroScreen;
use crate::backend::display::display_file::{MacroDisplay, SheetDisplay};

enum ScreenType{
    File,
    Sheet,
    Macro
}
impl Default for ScreenType{
    fn default() -> Self{
        ScreenType::Sheet
    }
}

pub struct Textexcel{
    toolbar: toolbar::ToolBar,
    selected_screen: ScreenType,
    macro_screen: MacroDisplay,
    sheet_screen: SheetDisplay

}
impl Application for Textexcel{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let macro_screen = MacroDisplay::new();
        let sheet_screen = SheetDisplay::new();
        (
            Self{
                toolbar: toolbar::ToolBar::default(),
                selected_screen: ScreenType::default(),
                macro_screen,
                sheet_screen
            },
            Command::none()
            )
    }

    fn title(&self) -> String {
        String::from("TextExcel-demo")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {

        match _message{
            Message::ViewMacroScreen =>{
                self.selected_screen = ScreenType::Macro;
            }
            Message::ViewFileScreen =>{
                self.selected_screen = ScreenType::File;
            }
            Message::ViewEditScreen =>{
                self.selected_screen = ScreenType::Sheet
            }
            Message::MacroScreen(ms)=> self.macro_screen.update(ms),
            _=> {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(8)
            .spacing(8)
            .align_items(Align::Center)
            .push(self.toolbar.view())
            .push(match &self.selected_screen{
                ScreenType::Macro => Container::new(
                    self.macro_screen.view()
                ),
                ScreenType::File =>{Container::new(
                    Text::new("File")

                )},
                ScreenType::Sheet=>{
                    self.sheet_screen.view()
                }
            })
            .into()
    }
}


fn handle_hotkey(key_code: keyboard::KeyCode)-> Option<Message>{
    unimplemented!()
}