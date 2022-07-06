
# Termbox2 Bindings

Very simple Rust bindings to [termbox2](https://github.com/termbox/termbox2).

A simple example:

```rust
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
```
