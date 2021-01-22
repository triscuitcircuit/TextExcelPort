use iced::{Settings, Application};
use textexcel_gui::backend::mvc::Textexcel;

fn main()-> iced::Result{
    env_logger::init();
    Textexcel::run(Settings::default())
}
