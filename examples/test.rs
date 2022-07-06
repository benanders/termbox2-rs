
use termbox2::*;

fn main() {
	unsafe {
		tb_init();
		tb_clear();
		tb_present();
		let mut ev = Event {
			etype: 0,
			emod: 0,
			key: 0,
			ch: 0,
			w: 0,
			h: 0,
			x: 0,
			y: 0,
		};
		tb_poll_event(&mut ev);
		tb_shutdown();
	}
}
