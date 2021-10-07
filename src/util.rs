use std::cell::RefCell;
use std::io;
use std::io::Read;
use std::rc::Rc;

use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::Terminal;

use crate::emoto;

pub fn start_ui(emoto: Rc<RefCell<emoto::Emoto>>) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut asi = async_stdin();
    terminal.clear()?;
    terminal.hide_cursor()?;
    loop {
        let emoto = emoto.borrow();

        terminal.draw(|rect| emoto::ui::draw(rect, &emoto))?;
        for key in asi.by_ref().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    terminal.show_cursor()?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
