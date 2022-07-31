
use termbox2::{Term, Style, Color};

fn main() {
    let term = Term::new().unwrap();
    term.clear();
    term.print(0, 0, "hello", Style::None, Color::Default, Color::Default);
    term.present();
    term.poll_event();
}
