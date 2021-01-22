use iced::{Container,Text};
use crate::backend::display::display_file as display;
use crate::backend::messages::{Message, MacroMessage};
use crate::backend::display::display_file::MacroDisplay;

pub struct MacroScreen{
    s_view: Option<display::MacroDisplay>
}

impl MacroScreen{
    pub fn new(s_view: Option<display::MacroDisplay>)->Self{
        Self{s_view}
    }
    pub fn view(&mut self)-> Container<Message>{
        match &mut self.s_view{
            None => {Container::new(Text::new("Couldn't load macros"))}
            Some(s_view) => {s_view.view()}
        }
    }
    pub fn update(&mut self, smessage: MacroMessage ){
        match &mut self.s_view{
            None => {}
            Some(s_view) => s_view.update(smessage)
        }
    }
}