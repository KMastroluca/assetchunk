use std::io;
use crossterm::cursor;
use crossterm::queue;
use crossterm::style;
use crossterm::terminal;
use crossterm::execute;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};


pub enum MenuState {
   Main,
   Pack,
   Unpack,
   List,
   Exit
}

pub struct ScreenStack {
   pub stack: Vec<MenuState>
}

impl ScreenStack {
   pub fn new() -> Self {
      Self {
         stack: vec![MenuState::Main]
      }
   }

   pub fn push(&mut self, state: MenuState) {
      self.stack.push(state);
   }

   pub fn pop(&mut self) -> Option<MenuState> {
      self.stack.pop()
   }

   pub fn peek(&self) -> Option<&MenuState> {
      self.stack.last()
   }
}

const MENU: &str = r#" 
+--------+-----+---------------------------------------------------------------------------+
|  Menu  | Key |                                Description                                |
+--------+-----+---------------------------------------------------------------------------+
| Pack   | p   |   Collect all files in a given directory and pack them into a .asset file |
| Unpack | u   |   Unpack an existing .asset file to a given directory                     |
| List   | l   |   List the assets packed within the .asset file.                          |
+--------+-----+---------------------------------------------------------------------------+
"#;



pub fn interactive_term<W>(w: &mut W) -> std::io::Result<()>
where 
   W: io::Write, {
   let mut screen_stack = ScreenStack::new();

   execute!(w, EnterAlternateScreen).unwrap();

   terminal::enable_raw_mode();

   loop {
      match screen_stack.peek() {
         Some(MenuState::Main) => {
            if draw_main_menu(w, &mut screen_stack).is_err() {
               eprintln!("[-] Error drawing main menu");
               break;
            }
         },
         Some(MenuState::Pack) => {
            println!("Pack");
         },
         Some(MenuState::Unpack) => {
            println!("Unpack");
         },
         Some(MenuState::List) => {
            println!("List");
         },
         Some(MenuState::Exit) => {
            break;
         },
         None => {},
      }   
   }


   execute!(w,
      style::ResetColor,
      cursor::Show,
      terminal::LeaveAlternateScreen
   ).unwrap();

   terminal::disable_raw_mode()


}



pub fn read_char() -> std::io::Result<char> {
   loop {
      if let Ok(Event::Key(KeyEvent {
         code: KeyCode::Char(c),
         kind: KeyEventKind::Press,
         modifiers: _,
         state: _,
      })) = event::read() {
         return Ok(c);
      }
   }
}

pub fn buffer_size() -> std::io::Result<(u16, u16)> {
   terminal::size()
}


pub fn draw_main_menu<W>(w: &mut W, screen_stack:&mut ScreenStack) -> std::io::Result<()>
where 
   W: io::Write, {
   loop {

      queue!(w,
      terminal::Clear(ClearType::All),
      cursor::Hide,
      cursor::MoveTo(1, 1),).unwrap();

      for LINE in MENU.split("\n") {
         queue!(w, style::Print(LINE), cursor::MoveToNextLine(1)).unwrap();
      }

      w.flush().unwrap();
   
      match read_char().unwrap() {
         'p' => {
            // Switch to pack state
            screen_stack.push(MenuState::Pack);
            break;
         },
         'u' => {
            screen_stack.push(MenuState::Unpack);
            break;
         },
         'l' => {
            screen_stack.push(MenuState::List);
            break;
         },
         'q' => {
            execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
            screen_stack.push(MenuState::Exit);
            break;
         },
         _ => {break;}
      };

   }


   Ok(())
}
