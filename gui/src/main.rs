use iced::{Settings, Application};
use gui::backend::mvc::Textexcel;

fn main()-> iced::Result{
    Textexcel::run(Settings::default())
}
