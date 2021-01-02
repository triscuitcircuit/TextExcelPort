#![feature(core_intrinsics)]
use async_trait::async_trait;
use crossterm::{cursor, event, execute, style, terminal, tty::IsTty, QueueableCommand, ErrorKind};
use textexcelport_core::{console,grid};
use crossterm::event::KeyCode;
use textexcelport_core::console::{Position, ClearType, Key};
use std::{
    cell::RefCell,cmp::Ordering, collections::VecDeque,
    env,io::{self,Write},path::Path,rc::Rc
};


/// Converts a 'Crossterm:;ErrorKind' to an 'io::Error' for the terminal
fn crossterm_error_to_io_error(e: crossterm::ErrorKind)-> io::Error{
    match e{
        crossterm::ErrorKind::IoError(e)=>e,
        crossterm::ErrorKind::Utf8Error(e)=>{
            io::Error::new(io::ErrorKind::InvalidData, format!("{}",e))
        }
        _ => io::Error::new(io::ErrorKind::Other, format!("{}",e)),
    }

}
/// gets environment variable
fn get_env_var_var_as_usize(name: &str)-> Option<usize>{
    match env::var_os(name){
        None => None,
        Some(value)=> value.as_os_str().to_string_lossy().parse::<usize>().map(Some).unwrap_or(None)
    }
}
pub struct TextConsole{
    is_tty: bool,
    buffer: VecDeque<console::Key>,
    need_line_flush: bool,
}
impl TextConsole{
    /// creates a new console based on the properties of stdin/stdout.
    pub fn from_stdio() -> io::Result<Self>{
        let is_tty = io::stdin().is_tty() && io::stdout().is_tty();
        if is_tty{
            terminal::enable_raw_mode().map_err(crossterm_error_to_io_error)?;
        }
        Ok(Self{is_tty, buffer: VecDeque::default(), need_line_flush: false})
    }
}
impl Drop for TextConsole{
    fn drop(&mut self) {
        if self.is_tty{
            terminal::disable_raw_mode().unwrap();
        }
    }
}
impl TextConsole{
    fn line_to_keys(s: String)->VecDeque<console::Key>{
        let mut keys = VecDeque::default();
        for ch in s.chars(){
            if ch == '\x1b'{
                keys.push_back(console::Key::Escape);
            }else if ch == '\n'{
                keys.push_back(console::Key::NewLine);
            }else if ch == '\r'{
                // ignore
            }else if !ch.is_control(){
                keys.push_back(console::Key::Char(ch))
            }else{
                keys.push_back(console::Key::Unknown(format!("{}",ch)));
            }
        }
        keys
    }
    fn read_key_from_tty(&mut self)-> io::Result<console::Key>{
        loop{
            if let event::Event::Key(ev) = event::read().map_err(crossterm_error_to_io_error)?{
                return match ev.code {
                    KeyCode::Backspace => Ok(console::Key::Backspace),
                    KeyCode::Enter => Ok(console::Key::NewLine),
                    KeyCode::Left => Ok(console::Key::ArrowLeft),
                    KeyCode::Right => Ok(console::Key::ArrowRight),
                    KeyCode::Up => Ok(console::Key::ArrowUp),
                    KeyCode::Down => Ok(console::Key::ArrowDown),
                    KeyCode::Esc => Ok(console::Key::Escape),
                    KeyCode::Char('c') if ev.modifiers == event::KeyModifiers::CONTROL => {
                        Ok(console::Key::Interrupt)
                    }
                    KeyCode::Char(ch) => Ok(console::Key::Char(ch)),
                    _ => Ok(console::Key::Unknown(format!("{:?}", ev))),
                }
            }
        }
    }
}
#[async_trait(?Send)]
impl console::Console for TextConsole{
    fn clear(&mut self, how: ClearType) -> io::Result<()> {
        let how = match how{
            ClearType::All => terminal::ClearType::All,
            ClearType::CurrentLine => terminal::ClearType::CurrentLine,
            ClearType::UntilNewLine => terminal::ClearType::UntilNewLine
        };
        let mut output = io::stdout();
        output.queue(terminal::Clear(how)).map_err(crossterm_error_to_io_error)?;
        if how == terminal::ClearType::All{
            output.queue(cursor::MoveTo(0,0)).map_err(crossterm_error_to_io_error)?;
        }
        output.flush();
        Ok(())
    }

    fn color(&mut self, foreground: Option<u8>, background: Option<u8>) -> io::Result<()> {
        let mut output = io::stdout();
        let foreground = match foreground{
            None => style::Color::Reset,
            Some(color) => style::Color::AnsiValue(color)
        };
        let background = match background{
            None => style::Color::Reset,
            Some(color) => style::Color::AnsiValue(color)
        };
        output.queue(style::SetForegroundColor(foreground)).map_err(crossterm_error_to_io_error)?;
        output.queue(style::SetBackgroundColor(background)).map_err(crossterm_error_to_io_error)?;
        self.need_line_flush = background != style::Color::Reset;
        Ok(())
    }

    fn enter_alt(&mut self) -> io::Result<()> {
        execute!(io::stdout(), terminal::EnterAlternateScreen).map_err(crossterm_error_to_io_error)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(io::stdout(), cursor::Hide).map_err(crossterm_error_to_io_error)
    }

    fn locate(&mut self, pos: Position) -> io::Result<()> {
        if pos.row > std::u16::MAX as usize{
            return Err(io::Error::new(io::ErrorKind::Other,"Row out of range"));
        }
        let column = pos.column as u16;
        let row = pos.row as u16;
        execute!(io::stdout(), cursor::MoveTo(column,row)).map_err(crossterm_error_to_io_error)
    }
    fn print(&mut self, text: &str) -> io::Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(text.as_bytes())?;
        if self.need_line_flush{
            execute!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))
                .map_err(crossterm_error_to_io_error)?;
        }
        stdout.write_all(b"\r\n")?;
        Ok(())
    }

    fn is_interactive(&self) -> bool {
        self.is_tty
    }

    fn leave_alt(&mut self) -> io::Result<()> {
        execute!(io::stdout(),terminal::LeaveAlternateScreen).map_err(crossterm_error_to_io_error)
    }

    fn move_within_line(&mut self, off: i16) -> io::Result<()> {
        match off.cmp(&0){
            Ordering::Less => execute!(io::stdout(),cursor::MoveLeft(-off as u16)),
            Ordering::Equal=>Ok(()),
            Ordering::Greater=> execute!(io::stdout(), cursor::MoveRight(off as u16)),
        }.map_err(crossterm_error_to_io_error)
    }

    async fn read_key(&mut self) -> io::Result<Key> {
        self.read_key_from_tty()
    }

    fn size(&self) -> io::Result<Position> {
        let lines = get_env_var_var_as_usize("LINES");
        let columns = get_env_var_var_as_usize("COLUMNS");
        let size = match (lines, columns){
            (Some(l), Some(c)) => console::Position{row: l, column:c},
            (l,c)=>{
                let (actual_columns, actual_lines)=
                    terminal::size().map_err(crossterm_error_to_io_error)?;
                console::Position{
                    row: l.unwrap_or(actual_lines as usize),

                    column: c.unwrap_or(actual_columns as usize)
                }
            }
        };
        Ok(size)
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(bytes)?;
        stdout.flush()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(io::stdout(),cursor::Show).map_err(crossterm_error_to_io_error)
    }
}