
use termbox2::*;

fn main() {
	let mut ev = Event::default();
	unsafe {
		tb_init();
		tb_clear();
		tb_present();
		tb_poll_event(&mut ev);
		tb_shutdown();
	}
}
