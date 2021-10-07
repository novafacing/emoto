use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::emoto::Emoto;

pub fn draw<B>(rect: &mut Frame<B>, _emoto: &Emoto)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from("Emoto\n"),
        Spans::from("A test thingy.\n"),
    ])
    .block(
        Block::default()
            .title("Emoto")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    )
    .alignment(Alignment::Center)
}

fn check_size(rect: &Rect) {
    if rect.width < 52 || rect.height < 28 {
        panic!("Window too small.");
    }
}
