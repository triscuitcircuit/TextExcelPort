use iced::{widget, Align, Button, Column, Container, Length, Row, Text, TextInput, scrollable, HorizontalAlignment};
use iced_native::{button, text_input, Widget};
use crate::backend::messages::{Message, MacroMessage, SheetMessage};
use crate::backend::styles;

enum MacroState{
    Loading,
    //Loaded(State),
    Saving,
    Type
}
pub struct SelectableButton {
    sheet_name: String,
    sheet_id: usize,
    button_state: button::State
}
impl SelectableButton{
    fn new(sheet_id: usize, sheet_name: String) -> Self{
        Self{
            sheet_name,
            sheet_id,
            button_state: Default::default()
        }
    }

    fn view(&mut self)-> Button<Message>{
        Button::new(
            &mut self.button_state,
            Text::new(self.sheet_name.to_string())
                .horizontal_alignment(HorizontalAlignment::Left)
        )
            //TODO: on press
            .width(Length::Fill)
            .style(styles::Button::SheetList {selected: false})
    }
}

enum SheetState{
    Loading,
    Saving,
    Typing
}
pub struct SheetDisplay{
    state: SheetState,
    formula_bar: text_input::State,
    scroll: scrollable::State,
    input_button: button::State,
    formula: String,
    work_sheet: Vec<SelectableButton>,
    cells: Vec<CellDisplay>
}
pub struct CellDisplay{
    cell_id: usize,
    input: text_input::State,
    text: String
}
impl CellDisplay{
    fn new(cell_id: usize)-> Self{
        Self{
            cell_id,
            input: Default::default(),
            text: String::from("")
        }
    }
    fn view(&mut self)-> TextInput<Message>{
        TextInput::new(&mut self.input,"test", &self.text,|s|{
            Message::SheetScreen(SheetMessage::Changed(s))
        }).width(Length::Fill)
    }

}
impl SheetDisplay{
    pub fn new()-> Self{
        Self{
            state: SheetState::Loading,
            formula_bar: Default::default(),
            scroll: Default::default(),
            input_button: Default::default(),
            formula: String::from(""),
            work_sheet: {
                let mut a: Vec<SelectableButton> = vec![];
                a.push(SelectableButton::new(0,"Sheet1".to_string()));
                a
            },
            cells: vec![]
        }
    }
    pub fn view(&mut self) -> Container<Message>{
        Container::new(
            Row::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(self.work_sheet[0].view())
                .push(
                    Text::new("No Sheet selected")
                )
        )

    }
    pub fn update(&mut self, message: SheetMessage){
        match message{
            SheetMessage::Changed(s) => {self.formula = s}
            SheetMessage::Read(s)=>{ self.formula = s}
            SheetMessage::Submitted(_) => {}
            SheetMessage::SheetHasBeenSaved => {}
        }
    }
}
pub struct MacroDisplay{
    state: MacroState,
    input: text_input::State,
    save_button: button::State,
    input_value: String
}

impl MacroDisplay{
    pub fn new()-> Self{
        Self{
            state: MacroState::Loading,
            input: Default::default(),
            save_button: Default::default(),
            input_value: String::from("")
        }
    }
    pub fn view(&mut self) -> Container<Message>{
        Container::new(
            Row::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(8)
                .push(
                    TextInput::new(&mut self.input,"test",&self.input_value,|s|{
                        Message::MacroScreen(MacroMessage::Changed(s))
                    })
                        .size(32)
                        .width(Length::Fill)
                )
                .push(
                    Button::new(&mut self.save_button,Text::new("Line"))
                        .style(styles::Button::Blue)
                        .padding(8)
                )
        )
    }
    pub fn update(&mut self, message: MacroMessage){
        match message{
            MacroMessage::Changed(s) => {
                self.input_value = s;
            }
            MacroMessage::Submitted(_) => {}
            MacroMessage::MacroHasBeenSaved => {}
        }
    }
}