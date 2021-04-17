use async_trait::async_trait;
use std::{
    io,
    //cell::RefCell,
    rc::Rc,
    ops::{Sub,Add}
};

#[derive(Clone,Debug)]
/// Decoded key presses as returned by the console
pub enum Key{
    /// Arrow key down
    ArrowDown,
    /// Arrow key left
    ArrowLeft,
    /// Arrow key right
    ArrowRight,
    /// Arrow key up
    ArrowUp,
    /// Backspace key
    Backspace,
    /// carriage return for current line
    CarriageReturn,
    ///printable character
    Char(char),
    /// terminal interrupt
    Interrupt,
    /// The escape key
    Escape,
    /// accepts a new line
    NewLine,
    /// an unknown sequence of Strings
    Unknown(String),
}
#[derive(Debug,Eq,PartialEq)]
pub enum ClearType{
    /// clear the whole console
    All,
    /// clears current line
    CurrentLine,
    /// clears until end of line
    UntilNewLine,
}
#[derive(Debug,Eq,PartialEq, Clone, Copy)]
///position struct to represent cursor position
pub struct Position{
    pub row: usize,
    pub column: usize,
}
impl std::ops::Sub for Position{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position{
            row: self.row - rhs.row,
            column: self.column - rhs.column
        }
    }
}
impl std::ops::Add for Position{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position{
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}
#[async_trait(?Send)]
/// hooks to manipulate the console
pub trait Console{
    ///clears the whole console
    fn clear(&mut self, how: ClearType)-> io::Result<()>;
    /// changes background and foreground color
    /// if both are set to "None," both will be set to default
    fn color(&mut self, foreground: Option<u8>, background: Option<u8>) -> io::Result<()>;
    /// Enters an alternate console
    fn enter_alt(&mut self)-> io::Result<()>;
    ///Hides the cursor
    fn hide_cursor(&mut self) -> io::Result<()>;
    /// moves the cursor to a given position within the screen
    fn locate(&mut self, pos: Position) -> io::Result<()>;
    /// writes text to the console followed by newline
    fn print(&mut self, text: &str) -> io::Result<()>;
    /// returns true if the console is interactive
    fn is_interactive(&self)-> bool;
    /// leave alternate terminal
    fn leave_alt(&mut self)-> io::Result<()>;
    /// moves cursor within the given position. Positive moves right, negative moves left
    fn move_within_line(&mut self, off: i16)-> io::Result<()>;
    /// waits for and returns next key press
    async fn read_key(&mut self)-> io::Result<Key>;
    ///Queries the size of the console
    ///
    /// The returned position represents the first row and column that lays outside the console
    fn size(&self)-> io::Result<Position>;
    /// writes the raw bytes to the console
    fn write(&mut self, bytes: &[u8])-> io::Result<()>;
    /// Shows the cursor
    fn show_cursor(&mut self)-> io::Result<()>;
}
///Reads the line of text interactively from console with given prompt
async fn read_line_interactive(
    console: &mut dyn Console,
    prompt: &str,
    previous: &str,
)-> io::Result<String>{
    let mut line = String::from(previous);
    console.clear(ClearType::UntilNewLine)?;
    if !prompt.is_empty() || !line.is_empty(){
        console.write(format!("{}{}",prompt, line).as_bytes())?;
    }
    let width ={
        //Assumes prompt was printed at column 0, if that is not the case
        // then normal calculation would not work
        let console_size = console.size()?;
        // subtracts the prompt length to make sure that the column is 0
        console_size.column - prompt.len()
    };
    let mut pos = line.len();

    loop{
        match console.read_key().await?{
            Key::ArrowRight =>{
                if pos < line.len(){
                    console.move_within_line(1)?;
                    pos += 1;
                }
            }
            Key::ArrowLeft=>{
                if pos > 0{
                    console.move_within_line(-1)?;
                    pos -= 1;
                }
            }
            Key::Backspace=>{
                if pos > 0{
                    console.hide_cursor()?;
                    console.move_within_line(-1)?;
                    console.write(line[pos..].as_bytes())?;
                    console.write(&[b' '])?;
                    console.move_within_line(-((line.len()-pos) as i16 +1))?;

                }
            }
            Key::CarriageReturn=>{
                if cfg!(not(target_os ="windows")){
                    console.write(&[b'\r', b'\n'])?;
                    break;
                }
            }
            Key::Char(ch)=>{
                debug_assert!(line.len() < width);
                if pos < line.len(){
                    console.hide_cursor()?;
                    console.write(&[ch as u8])?;
                    console.move_within_line(-((line.len() -pos) as i16))?;
                }else{
                    console.write(&[ch as u8])?;
                }
                line.insert(pos,ch);
                pos += 1
            }
            Key::Escape=>{
                // ignored
            }
            Key::Interrupt=>{
                return Err(io::Error::new(io::ErrorKind::Interrupted,"Ctrl+C"))
            }
            Key::NewLine=>{
                console.write(&[b'\r',b'\n'])?;
                break;
            }
            Key::Unknown(_)=>(),
            _ => {()}
        }
    }
    Ok(line)
}
async fn read_line_raw(console: &mut dyn Console)-> io::Result<String>{
    let mut line = String::new();
    loop {
        match console.read_key().await?{
            Key::ArrowLeft| Key::ArrowDown| Key::ArrowRight =>(),
            Key::Backspace=>{
                if !line.is_empty(){
                    line.pop();
                }
            }
            Key::CarriageReturn=>{
                if cfg!(not(target_os = "windows")){
                    break;
                }
            }
            Key::Char(ch)=> line.push(ch),
            Key::Escape => (),
            Key::Interrupt => return Err(io::Error::new(io::ErrorKind::Interrupted,"Ctrl+C")),
            Key::NewLine => break,
            Key::Unknown(unknown) => line += &unknown,
            _ => {}
        }
    }
    Ok(line)
}
pub async fn read_line(
    console: &mut dyn Console,
    prompt: &str,
    previous: &str
) -> io::Result<String>{
    if console.is_interactive(){
        read_line_interactive(console, prompt, previous).await
    }else{
        read_line_raw(console).await
    }
}