use iced::{
    button, executor, keyboard, pane_grid, scrollable, Align, Application,
    Button, Column, Command, Container, Element, HorizontalAlignment, Length,
    PaneGrid, Scrollable, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};
use std::collections::HashMap;

struct Content{
    id: usize,
    scroll: scrollable::State,
    close: button::State
}
impl Content{
    fn new(id: usize)-> Self{
        Content{
            id,
            scroll: scrollable::State::new(),
            close: button::State::new(),
        }
    }
}
pub struct Workbook {
    panes: pane_grid::State<Content>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}
impl Application for Workbook {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (panes, _) = pane_grid::State::new(Content::new(0));
        (
            Workbook{
                panes,
                panes_created: 0,
                focus: None
            }, Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("TextExcelGraphical-demo")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        unimplemented!()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        unimplemented!()
    }
}