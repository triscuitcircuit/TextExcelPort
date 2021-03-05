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
use crate::backend::display::display_file::{MacroDisplay, SheetDisplay, SelectableButton, StatDisplay};
use iced_native::keyboard::KeyCode;
use textexcelport_core::statcrunching::iqr::iqr_calc_string;
use textexcelport_core::statcrunching::{parse, StatError};

enum ScreenType{
    File,
    Sheet,
    Macro,
    Stat
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
    stat_screen: StatDisplay,
    pub sheet_screen: SheetDisplay

}
impl Application for Textexcel{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let macro_screen = MacroDisplay::new();
        let sheet_screen = SheetDisplay::new();
        let stat_screen = StatDisplay::new();
        (
            Self{
                toolbar: toolbar::ToolBar::default(),
                selected_screen: ScreenType::default(),
                macro_screen,
                stat_screen,
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
            Message::RemoveSheet=>{
                let a = self.sheet_screen.work_sheet.pop();
            }
            Message::NewSheet=> {
                let name: String = format!("test {}",self.sheet_screen.sheets_created).to_string();
                self.sheet_screen.work_sheet.push(
                    SelectableButton::new(self.sheet_screen.sheets_created,name));
                self.sheet_screen.sheets_created += 1;
            }
            Message::ViewStatScreen=>{
                self.selected_screen = ScreenType::Stat
            }
            Message::IqrSubmit =>{
                let delimiter = self.stat_screen.delimiter_value.as_str().clone();
                self.stat_screen.output = if delimiter.is_empty(){
                    "No delimiter provided".to_string()
                }else {
                    if self.stat_screen.input_value.is_empty(){
                        "No values provided".to_string()
                    }else {
                        let a = parse(&*self.stat_screen.input_value, delimiter);
                        match iqr_calc_string(a){
                            Ok(e) => {e}
                            Err(e) => {format!("{:#?}",e)}
                        }
                    }
                }
            }
            Message::MacroScreen(ms)=> self.macro_screen.update(ms),
            Message::StatScreen(ms) => self.stat_screen.update(ms),
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
                ScreenType::Stat => Container::new(
                    self.stat_screen.view()
                ),
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
    match key_code{
        KeyCode::Copy => {None}
        KeyCode::LControl | KeyCode::Key1 => {None}
        _ => None,
    }
}