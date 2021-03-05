use iced::{Container,Text};
use crate::backend::display::display_file as display;
use crate::backend::messages::{Message, StatMessage};

pub struct StatScreen{
    s_view: Option<display::StatDisplay>
}

impl StatScreen{
    pub fn new(s_view: Option<display::StatDisplay>)->Self{
        Self{s_view}
    }
    pub fn view(&mut self)-> Container<Message>{
        match &mut self.s_view{
            None => {Container::new(Text::new("Couldn't load stat functionality"))}
            Some(s_view) => {s_view.view()}
        }
    }
    pub fn update(&mut self, smessage: StatMessage ){
        match &mut self.s_view{
            None => {}
            Some(s_view) => s_view.update(smessage)
        }
    }
}