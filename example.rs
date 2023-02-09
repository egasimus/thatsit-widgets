use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::{*, crossterm::style::Color};
use thatsit_widgets::*;

static EXITED: AtomicBool = AtomicBool::new(false);

fn main () {
    run(&EXITED, &mut std::io::stdout(), Example);
}

struct Example;

impl Widget for Example {

    impl_render!(self, out, area => {
        let yellow = |s: String|s.with(Color::Yellow);
        Border(Tall, Inset,  Styled(&yellow, " Inset Tall".into())).render(out, Area(1, 1, 14, 3))?;
        Border(Wide, Inset,  Styled(&yellow, " Inset Wide".into())).render(out, Area(1, 5, 14, 3))?;
        Border(Flat, Inset,  Styled(&yellow, " Inset Flat".into())).render(out, Area(1, 9, 14, 3))?;
        Border(Tall, Outset, Styled(&yellow, "Outset Tall".into())).render(out, Area(16, 1, 14, 3))?;
        Border(Wide, Outset, Styled(&yellow, "Outset Wide".into())).render(out, Area(16, 5, 14, 3))?;
        Border(Flat, Outset, Styled(&yellow, "Outset Flat".into())).render(out, Area(16, 9, 14, 3))?;
        Ok((0, 0))
    });

    impl_handle!(self, event => {
        EXITED.store(true, Ordering::Relaxed);
        Ok(true)
    });

}
