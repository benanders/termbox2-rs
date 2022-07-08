
use termbox2::{Term, Style, Color};

fn main() {
    let term = Term::new().unwrap();
    term.clear();
    term.set_cell(0, 0, 'H', Style::None, Color::Default, Color::Default);
    term.set_cell(1, 0, 'e', Style::None, Color::Default, Color::Default);
    term.set_cell(2, 0, 'l', Style::None, Color::Default, Color::Default);
    term.set_cell(3, 0, 'l', Style::None, Color::Default, Color::Default);
    term.set_cell(4, 0, 'o', Style::None, Color::Default, Color::Default);
    term.present();
    term.poll_event();
}
