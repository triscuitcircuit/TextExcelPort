use iced::pane_grid;

#[derive(Debug, Clone)]
pub enum Message {
    Save,
    NewSheet,
    RemoveSheet,
    ViewMacroScreen,
    ViewFileScreen,
    ViewSaveScreen,
    ViewEditScreen,
    ViewStatScreen,
    MacroScreen(MacroMessage),
    IqrSubmit,
    DeviationSubmit,
    FileScreen(FileMessage),
    SheetScreen(SheetMessage),
    StatScreen(StatMessage)
    // Split(pane_grid::Axis, pane_grid::Pane),
    // SplitFocused(pane_grid::Axis),
    // FocusAdjacent(pane_grid::Direction),
    // Clicked(pane_grid::Pane),
    // Dragged(pane_grid::DragEvent),
    // Resized(pane_grid::ResizeEvent),
    // Close(pane_grid::Pane),
    // CloseFocused,
}
#[derive(Debug, Clone)]
pub enum MacroMessage{
    Changed(String),
    Submitted(bool),
    MacroHasBeenSaved
}
#[derive(Debug, Clone)]
pub enum StatMessage{
    DelimiterChange(String),
    ValuesChange(String),
    DeviationSubmit,
    IqrSubmit,
    MacroHasBeenSaved
}

#[derive(Debug, Clone)]
pub enum SheetMessage{
    Changed(String),
    Read(String),
    Submitted(bool),
    SheetHasBeenSaved
}
#[derive(Debug, Clone, Copy)]
pub enum FileMessage{
    SaveScreen(SaveMessage),
}
#[derive(Debug, Clone, Copy)]
pub enum SaveMessage{
    SheetHasBeenSaved
}