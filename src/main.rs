use std::env;
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self,ClearType},
    cursor,
    // style::{self,Stylize},
};

// main editor struct that holds the state
// currently tracks wether user wants to quit
// later will hold : cursor position, file content, etc.
struct Editor{
    should_quit: bool,
    should_write: bool,
    lines:Vec<String>,
    c_row:usize,
    c_col:usize,
    scroll_row:usize,
}

impl Editor{
    // createss a new editor instance with default state
    fn new()-> Self{
        Editor{
            c_col:0,
            c_row:0,
            scroll_row:0, 
            should_quit: false,
            should_write: false,
            lines: vec![
                    String::from("This is line 1: Hello, world!"),
                    String::from("This is line 2: Rust text editor test."),
                    String::from("This is line 3: Lorem ipsum dolor sit amet."),
                    String::from("This is line 4: Cursor movement check."),
                    String::from("This is line 5: End of dummy text."),     
            ],
        }
    }
    
    pub fn load_file(&mut self, path: &str)-> io::Result<()>{
        // load file using rusts std::fs::

    }

    // Initialise the editor env:
    // - switches terminal to "raw mode" no line buffering , no echo
    // clears entire screen 
    // draws a welcome screen 
    // basically takes over the terminal form shell
    fn init(&mut self)-> io::Result<()>{
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(self.c_row.try_into().unwrap(),self.c_col.try_into().unwrap()),
            cursor::SavePosition)?;
            // self.draw_welcome_screen()?;
        self.short_screen_redraw()?;
       Ok(())
    }


    // main event loop - the hart:
    // -refresh whats shown on screen
    // check if user wants to quit
    // wait for and process the next process_keypress
    // repeat until quit
    // this loop runs continuously while x5 is open
    fn run(&mut self)-> io::Result<()>{
        loop{
            self.refresh_screen()?;

            if self.should_quit{
                break;
            }
        
            self.process_keypress()?;
        }
        Ok(())
    }


   // this writes text into the terminal when clicked and 
   // also calls the function that updates cursor coords   
    fn write_text(&mut self, ch: &str )-> io::Result<()>{
        
        let  ( pos_x,  pos_y) = cursor::position().unwrap();
      //  print!("{}{}",pos_x,pos_y); 

        execute!(
            io::stdout(),
            cursor::Hide,
         //   terminal::BeginSynchronizedUpdate,
            )?;
        print!("{}",ch);

        execute!(io::stdout(),
       // terminal::EndSynchronizedUpdate,
        cursor::MoveTo(pos_x+1,pos_y),
        // cursor::SavePosition,
        cursor::Show)?;
        
        self.update_cursor_position_bar()?;
        
        io::stdout().flush()?;

        Ok(())
    } 
    fn update_cursor_position_bar(&mut self)-> io::Result<()>{
            let (w,h) = terminal::size()?;
            let (x,y) = cursor::position().unwrap();

            execute!(
                io::stdout(),
                cursor::Hide,
                cursor::SavePosition,
                cursor::MoveTo(w-20,h))?;
            print!("@ {} , {}",x,y);

            execute!(
                io::stdout(),
                cursor::RestorePosition,
                cursor::Show)?;

            Ok(())
    } 
   

    // process keypress by user
    // reads next keyboard event (blocking call , waits for input)
    // matches against different key types
    // takes appropriate actions 
    // this is wehre all keyboard interaction lives 
    fn process_keypress(&mut self)-> io::Result<()>{
        let event  = event::read()?;

        if let Event::Key(KeyEvent {code, modifiers, ..}) = event{
            match code{
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL)=>{
                    self.should_quit = true;
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL)=>{
                   self.clear_screen()?;
                }
                KeyCode::Up => {
                    self.move_up()?;
                }
                KeyCode::Down =>{
                    self.move_down()?;
                }
                KeyCode::Left =>{
                    self.move_left()?;
                }
                KeyCode::Right =>{
                    self.move_right()?;
                }
                KeyCode::Char(c) =>{
                 self.write_text(&format!("{}",c))?;
                // self.show_message(&format!("char pressed:{}",c))?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    
        fn move_down(&mut self)->io::Result<()>{
            if self.c_row + 1 < self.lines.len() {
                self.c_row+=1;
                self.c_col = self.c_col.min(self.lines[self.c_row].len());
            }
            Ok(())
        }
        fn move_right(&mut self)-> io::Result<()>{
            let line_len = self.lines[self.c_row].len();

            if self.c_col < line_len{
                self.c_col +=1;
            }

            Ok(())
        }
        fn move_left(&mut self)->io::Result<()>{
            if self.c_col>0{
                self.c_col-=1;
            }
            Ok(())
        } 
        fn move_up(&mut self)-> io::Result<()>{
            if self.c_row>0{
                self.c_row-=1;
                self.c_col = self.c_col.min(self.lines[self.c_row].len());
            }
            Ok(())
        }

    // refresh whats displayed
    // hide cursor (prevent flicker during redraw)
    // move cursor to top left
    // flushes output buffer 
    // show cursor again
    fn refresh_screen(&mut self) -> io::Result<()>{
      
        let x = self.c_col as u16;
        let y = self.c_row as u16;
        let line_len = self.lines[self.c_row].len();
        let (w,h) = terminal::size()?;
        if line_len > h.into() {
            if h>0{
                self.scroll_row = h.into();
            }
            self.scroll_row =0;
            self.redraw_screen()?;
        }
        execute!(
            io::stdout(),
            cursor::Hide,
            cursor::MoveTo(x,y),
             cursor::Show,
            )?;
        Ok(())
    }


    // draw intial welcm screen when editor opens 
    // - get terminal dimn's 
    // center the messg 
    // use cursor position to place text at specific coords
    // only called once during init not in the main loop
    fn redraw_screen(&mut self)->io::Result<()>{
        let (x,y) = terminal::size()?; 
        for i in 0..y{
             let sec_i = i as usize;
            println!("{}",self.lines[self.scroll_row+sec_i]);
        }
        io::stdout().flush()?;
        Ok(())
    }

    fn short_screen_redraw(&self)-> io::Result<()>{
        for i in 0..50{
            println!("{}",self.lines[0]);
            execute!( 
                io::stdout(),
                cursor::MoveTo(0,(i+1).try_into().unwrap()),)?;
            } 
        let x= self.c_col as u16;
        let y = self.c_row as u16;
        execute!(io::stdout(),cursor::MoveTo(x,y))?;
        io::stdout().flush()?;

        Ok(())
    }

    fn clear_screen(&self)-> io::Result<()>{
        execute!(
            io::stdout(),
            cursor::Hide,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0),
            cursor::Show,
            )?;
        io::stdout().flush()?;

        Ok(())
    }

    // show a temp msg at bttm of screen
    // gets terminal height to know where bottom is
    // move cursor to last row (height-1)
    // clears the line and prrints messg
    // used for debuggin showing what keys were presed 
    // will become a status bar later 
    fn show_message(&self, msg: &str) -> io::Result<()>{
        let (_, height) = terminal::size()?;

        execute!(
            io::stdout(),
            cursor::MoveTo(0,height-1),
            terminal::Clear(ClearType::CurrentLine),
            )?;
        print!("{}", msg);
        io::stdout().flush()?;
        execute!(
            io::stdout(),
            cursor::RestorePosition,
            )?;
        Ok(())
    }
    
    // clean up and restore terminal to normal state 
    // clear screen
    // move cursor to topleft
    // disable raw disable_raw_mode
    // must be called before exiting or terminal stays broken
    //
    fn cleanup(&self) -> io::Result<()>{
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0),
            )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

// implement dROP trait for safety - this is rusts destructer
// if the program panics or exits unexpectedly Drop ensures cleanup()
// still runs and terminal doest stay in raw mode 
// this is RAII pattern 
impl Drop for Editor {
    fn drop(&mut self){
        let _ = self.cleanup();
    }
}

 

// entry point -sets up x5
// create instance
// Initialise
// run main loop
// cleanup 
// print confirm messg
// the ? operator propagates errors upto the caller
fn main() -> io::Result<()> {
    
    // args[0] is always program name( like "x5" eventually)
    // args[1] is "filename.txt"
    // args[2] would be the next argument, etc.. 
    let filename = env::args().nth(1);

    let mut editor = Editor::new();

    match filename{
        Some(path) =>{
            editor.load_file(path),
        }
        None=>{
            start_empty()
            //do nothing
        }
    }
  
    editor.init()?;
    editor.run()?;
    editor.cleanup()?;

    println!("Editor exited!!");
    Ok(())
}
