use iced::pane_grid;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Save,
    ViewMacroScreen,
    ViewFileScreen,
    ViewSaveScreen,
    ViewEditScreen,
    MacroScreen(MacroMessage),
    FileScreen(FileMessage),
    SaveScreen(SaveMessage),
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}
#[derive(Debug, Clone, Copy)]
pub enum MacroMessage{
    MacroHasBeenSaved
}
#[derive(Debug, Clone, Copy)]
pub enum FileMessage{

}
#[derive(Debug, Clone, Copy)]
pub enum SaveMessage{
    SheetHasBeenSaved
}