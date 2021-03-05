use iced::{widget, Align, Button, Column, Container, Length, Row, Text, TextInput, scrollable, HorizontalAlignment, Scrollable};
use iced_native::{button, text_input, Widget};
use crate::backend::messages::{Message, MacroMessage, SheetMessage, StatMessage};
use crate::backend::styles;
use core::fmt::Alignment::Left;

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
    pub(crate)  fn new(sheet_id: usize, sheet_name: String) -> Self{
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
    new_sheet: button::State,
    pub sheets_created: usize,
    r_sheet: button::State,
    scroll: scrollable::State,
    input_button: button::State,
    formula: String,
    pub work_sheet: Vec<SelectableButton>,
    pub cells_scrollable: scrollable::State,
    input_value: String,
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
            new_sheet: Default::default(),
            sheets_created: 0,
            r_sheet: Default::default(),
            scroll: Default::default(),
            input_button: Default::default(),
            formula: String::from(""),
            work_sheet: {
                let mut a: Vec<SelectableButton> = vec![];
                a.push(SelectableButton::new(0,"Sheet1".to_string()));
                a.push(SelectableButton::new(1,"Sheet2".to_string()));
                a.push(SelectableButton::new(2,"Sheet3".to_string()));
                a
            },
            cells_scrollable: Default::default(),
            input_value: "".to_string(),
            cells: vec![]
        }
    }
    pub fn view(&mut self) -> Container<Message>{
        let sheet_list = Container::new(

            self.work_sheet.iter_mut().fold(
                Row::new()
                    .push(Button::new(&mut self.new_sheet,Text::new("New Sheet")
                        .size(22))
                        .on_press(Message::NewSheet)
                        .style(styles::Button::Blue))
                    .push(Button::new(&mut self.r_sheet, Text::new("Delete")
                        .size(22))
                        .on_press(Message::RemoveSheet)
                        .style(styles::Button::Blue)
                    )
                    ,
                |s,b|s.push(b.view())
            )
        );
        // let list = Container::new(
        //     self.cells.iter_mut().fold(
        //         Scrollable::new(&mut self.cells_scrollable)
        //             .spacing(8),
        //         |s,b| s.push(b.view()),
        //     )
        // )
        //     .style(styles::Container::Basic)
        //     .height(Length::FillPortion(9))
        //     .padding(24);
        let formula_bar =Container::new(
            Row::new()
                // .height(Length::Fill)
                // .width(Length::Fill)
                //.spacing(8)
                .height(Length::Units(30))
                .push(
                    TextInput::new(&mut self.formula_bar,
                                   "formula",&self.input_value,
                                   |s|{Message::Save}))
                .push(
                    Button::new(&mut self.input_button,Text::new("F(x)")
                    )
                        .style(styles::Button::Blue)
                        .on_press(Message::NewSheet)
                ));
        let list_display = Container::new(
            Row::new().push(sheet_list).padding(8)
        );
            Container::new(
                Column::new()
                    .push(list_display)
                    .push(formula_bar)
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
enum StatState{
    Loading,
    Saving,
    Type
}
pub struct StatDisplay{
    state: StatState,
    input: text_input::State,
    pub input_value: String,
    delimiter_input: text_input::State,
    pub delimiter_value: String,
    iqr_button: button::State,
    standard_deviation: button::State,
    pub output: String
}
impl StatDisplay{
    pub fn new()-> Self{
        Self{
            state: StatState::Loading,
            input: Default::default(),
            input_value: "".to_string(),
            delimiter_input: Default::default(),
            delimiter_value: "".to_string(),
            iqr_button: Default::default(),
            standard_deviation: Default::default(),
            output: "".to_string()
        }
    }
    pub fn view(&mut self) -> Container<Message>{
        let button_container = Container::new(
            Row::new()
                .height(Length::Units(32))
                .push(
                    TextInput::new(&mut self.delimiter_input,"delimiter",&self.delimiter_value,|s|{
                        Message::StatScreen(StatMessage::DelimiterChange(s))
                    })
                        .size(32)
                        .width(Length::Fill)
                )
                .push(
                    Button::new(&mut self.iqr_button, Text::new("IQR Calc"))
                        .on_press(Message::IqrSubmit)
                        .style(styles::Button::Blue)
                        .padding(8)
                )
                .push(
                    Button::new(&mut self.standard_deviation,Text::new("S_Deviation")
                    )
                        .on_press(Message::DeviationSubmit)
                        .style(styles::Button::Blue)
                        .padding(8)
                )
        );
        let input = Container::new(
            Row::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(8)
                .push(
                    TextInput::new(&mut self.input,"values",&self.input_value,|s|{
                        Message::StatScreen(StatMessage::ValuesChange(s))
                    })
                        .size(32)
                        .width(Length::Fill)
                )

        );
        let output = Container::new(
          Row::new()
              .height(Length::Fill)
              .width(Length::Fill)
              .align_items(Align::Center)
              .spacing(8)
              .push(Text::new(
                  if self.output.is_empty(){
                      "no output"}else { self.output.as_str() }))
        );
        Container::new(
            Column::new()
                .push(button_container)
                .push(input)
                .push(output)
        )

    }
    pub fn update(&mut self, message: StatMessage){
        match message{
            StatMessage::DelimiterChange(s) => {
                self.delimiter_value = s
            }
            StatMessage::ValuesChange(s) => {
                self.input_value = s
            }
            StatMessage::DeviationSubmit => {}
            StatMessage::IqrSubmit => {
                self.output = self.delimiter_value.clone();
            }
            StatMessage::MacroHasBeenSaved => {}
        }
    }
}