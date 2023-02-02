use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::*;
use thatsit_widgets::*;

static EXITED: AtomicBool = AtomicBool::new(false);

fn main () {
    run(&EXITED, &mut std::io::stdout(), Example);
}

struct Example;

impl Widget for Example {
    impl_render!(self, out, area => {
        Border(InsetTall, "Inset Tall").render(out, Area(1, 1, 12, 5))?;
        Border(InsetWide, "Inset Wide").render(out, Area(1, 7, 12, 5))?;
        Ok((0, 0))
    });
    impl_handle!(self, event => {
        EXITED.store(true, Ordering::Relaxed);
        Ok(true)
    });
}
