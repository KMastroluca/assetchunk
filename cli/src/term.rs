use std::io;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use crossterm::cursor;
use crossterm::queue;
use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::execute;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, EventStream, KeyEventKind};

use std::io::{stdout, Write};

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


pub struct InteractiveTermRecievers {
   pub carat_blinker: Receiver<bool>,
   pub event: Receiver<crossterm::event::Event>
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


pub fn interactive_term<W>(w: &mut W, recievers:&InteractiveTermRecievers) -> std::io::Result<()>
where 
   W: io::Write, {

   let mut screen_stack = ScreenStack::new();

   execute!(w, EnterAlternateScreen).unwrap();

   terminal::enable_raw_mode();

   loop {
      match screen_stack.peek() {
         Some(MenuState::Main) => {
            if draw_main_menu(w, &mut screen_stack, recievers).is_err() {
               eprintln!("[-] Error drawing main menu");
               break;
            }
         },
         Some(MenuState::Pack) => {
            if draw_pack_menu(w, &mut screen_stack, recievers).is_err() {
               eprintln!("[-] Error drawing pack menu");
               break;
            }
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



pub fn buffer_size() -> std::io::Result<(u16, u16)> {
   terminal::size()
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InputMode {
   Nav,
   Input(InputField)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InputField {
   InputDir,
   OutputName
}

pub fn carat_blinker(sender: Sender<bool>) {
   let mut visible = false;
   loop {
      thread::sleep(Duration::from_millis(500));
      visible = !visible;
      if let Err(_) = sender.send(visible) {
         eprintln!("[-] Error: Failed To Send Carat Blinker Message");
         break;
      }
   }
}

pub fn draw_pack_menu<W>(w:&mut W, screen_stack:&mut ScreenStack, recievers:&InteractiveTermRecievers) -> std::io::Result<()>
where 
   W: io::Write, {
      

      let mut input_mode = InputMode::Nav;
   
      let mut selection = 0;
      let mut cursor_pos = (0, 0);
      let mut input_dir = String::new();
      let mut output_name = String::new();

      
      let mut cursor_visible = true;

      loop {

         if let Ok(visible) = recievers.carat_blinker.try_recv() {
            cursor_visible = visible;
         } 

         queue!(w, 
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0),
         ).unwrap();





         match selection {
            1 => {
               queue!(w, 
                  style::PrintStyledContent(format!("=> Input Directory: {}", input_dir).blue())).unwrap();
               if cursor_visible && input_mode == InputMode::Input(InputField::InputDir) {
                  queue!(w, style::PrintStyledContent("|".white())).unwrap();
               } else {
                  queue!(w, style::PrintStyledContent("".white())).unwrap();
               }
               queue!(w, cursor::MoveToNextLine(1)).unwrap();

               queue!(w, cursor::MoveRight(3), style::Print(format!("Output Name: {}", output_name)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::Print("Pack Assets"), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::PrintStyledContent("Go Back".red()), cursor::MoveToNextLine(1)).unwrap();
            },
            2 => {
               queue!(w, cursor::MoveRight(3), style::Print(format!("Input Directory: {}", input_dir)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, 
                  style::PrintStyledContent(format!("=> Output Name: {}", output_name).blue())).unwrap();
               if cursor_visible && input_mode == InputMode::Input(InputField::OutputName) {
                  queue!(w, style::PrintStyledContent("|".white())).unwrap();
               } else {
                  queue!(w, style::PrintStyledContent("".white())).unwrap();
               }
               queue!(w, cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::Print("Pack Assets"), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::PrintStyledContent("Go Back".red()), cursor::MoveToNextLine(1)).unwrap();
            },
            3 => {
               queue!(w, cursor::MoveRight(3), style::Print(format!("Input Directory: {}", input_dir)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::Print(format!("Output Name: {}", output_name)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, style::PrintStyledContent("=> Pack Assets".blue()), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::PrintStyledContent("Go Back".red()), cursor::MoveToNextLine(1)).unwrap();
            }
            0 => {
               queue!(w, cursor::MoveRight(3), style::Print(format!("Input Directory: {}", input_dir)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::Print(format!("Output Name: {}", output_name)), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, cursor::MoveRight(3), style::Print("Pack Assets"), cursor::MoveToNextLine(1)).unwrap();
               queue!(w, style::PrintStyledContent("=> Go Back".red()), cursor::MoveToNextLine(1)).unwrap();
            }
            _ =>{}
         }

         // Calculate The Divider
         let mut divider_text = String::from("__DETAILS");
         let divider_length = divider_text.len() + (buffer_size().unwrap().0 as usize - (divider_text.len() * 2));
         for _ in 0..divider_length {
            divider_text.push('_');
         }

         queue!(w, cursor::MoveTo(0, buffer_size().unwrap().1 - 6), style::Print(divider_text)).unwrap();

         // Show Asset File Name & Manifest File Name + Path
         if output_name.len() > 0 {
            queue!(w, cursor::MoveTo(3, buffer_size().unwrap().1 - 5), style::PrintStyledContent(format!("Asset Chunk File: {}.chunk.asset", output_name).magenta())).unwrap();
            queue!(w, cursor::MoveTo(3, buffer_size().unwrap().1 - 4), style::PrintStyledContent(format!("Asset Manifest File: {}.manifest.asset", output_name).magenta())).unwrap();
         }





         w.flush().unwrap();

         if let Ok(Event::Key(event)) = recievers.event.try_recv() {

            match event.kind {

               KeyEventKind::Release => {

                  if input_mode == InputMode::Nav {

                     match event.code {

                        KeyCode::Up => {
                           // Move up
                           if selection == 0 {
                              selection = 3;
                           } else {
                              selection -= 1;
                           }
                        },

                        KeyCode::Down => {
                           // Move down
                           if selection == 3 {
                              selection = 0;
                           } else {
                              selection += 1;
                           }
                        },

                        KeyCode::Enter => {
                           // Enter
                           match selection {
                              0 => {
                                 // Go Back
                                 screen_stack.pop();
                              },
                              1 => {
                                 // Input Dir
                                 input_mode = InputMode::Input(InputField::InputDir);
                              },
                              2 => {
                                 // Output Name
                                 input_mode = InputMode::Input(InputField::OutputName);
                              },
                              3 => {
                                 // Pack Assets
                                 println!("Pack Assets");
                              },
                              _ => {}
                           }
                        },

                        _ => {}

                     }

                  } else if  input_mode == InputMode::Input(InputField::InputDir) {

                     match event.code {

                        KeyCode::Esc => {
                           input_mode = InputMode::Nav;
                        },

                        KeyCode::Backspace => {
                           input_dir.pop();
                        },

                        KeyCode::Char(c) => {
                           input_dir.push(c);
                        },

                        _ => {}

                     }

                  } else if input_mode == InputMode::Input(InputField::OutputName) {

                     match event.code {

                        KeyCode::Esc => {
                           input_mode = InputMode::Nav;
                        },

                        KeyCode::Backspace => {
                           output_name.pop();
                        },

                        KeyCode::Char(c) => {
                           output_name.push(c);
                        },

                        _ => {}

                     }
                      
                  }
               },
               _ => {}
            }
         }


      }
}

pub fn draw_main_menu<W>(w: &mut W, screen_stack:&mut ScreenStack, recievers:&InteractiveTermRecievers) -> std::io::Result<()>
where 
   W: io::Write, {
   loop {

      queue!(w,
      terminal::Clear(ClearType::All),
      cursor::Hide,
      cursor::MoveTo(1, 1),).unwrap();

      for line in MENU.split("\n") {
         queue!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap();
      }

      w.flush().unwrap();
   
      if let Ok(Event::Key(event)) = recievers.event.try_recv() {
         match event.kind {
            KeyEventKind::Release => {
               match event.code {
                  KeyCode::Char(c) => {
                     match c {
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
                  },
                  _ => {}
               }
            },
            _ => {}
         }

      };

   }


   Ok(())
}
   