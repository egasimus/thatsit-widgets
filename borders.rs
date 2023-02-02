use crate::{*, crossterm::{cursor::*,style::*}};

/// A border around another widget
#[derive(Copy, Clone, Default)]
pub struct Border<T: BorderStyle, W: Widget>(pub T, pub W);

impl<T: BorderStyle, W: Widget> Widget for Border<T, W> {
    impl_render!(self, out, area => {
        let Area(x, y, w, h) = area;
        if w == 0 || h == 0 { return Ok((0, 0)) }

        fn set_colors (out: &mut dyn Write, inverse: bool) -> Result<()> {
            let bg = Color::AnsiValue(235);
            if inverse {
                out
                    .queue(SetBackgroundColor(Color::AnsiValue(16)))?
                    .queue(SetForegroundColor(bg))?;
            } else {
                out
                    .queue(SetBackgroundColor(bg))?
                    .queue(SetForegroundColor(Color::AnsiValue(240)))?;
            }
            Ok(())
        }

        //Filled(bg).layout(max)?.render(term, Area(Point(x, y), Size(w, h)))?;
        
        // draw top
        let (top_left, inverse) = T::top_left();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x, y))?.queue(Print(&top_left))?;

        let (top, inverse) = T::top();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x+1, y))?.queue(Print(&String::from(top).repeat((w-2) as usize)))?;

        let (top_right, inverse) = T::top_right();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x+w-1, y))?.queue(Print(&top_right))?;

        // draw sides and background
        let (left, inverse) = T::left();
        set_colors(out, inverse)?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x, y))?.queue(Print(&left))?;
        }

        out
            .queue(SetBackgroundColor(Color::AnsiValue(235)))?
            .queue(SetForegroundColor(Color::AnsiValue(240)))?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x+1, y))?.queue(Print(&" ".repeat((w-2) as usize)))?;
        }

        let (right, inverse) = T::right();
        set_colors(out, inverse)?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x+w-1, y))?.queue(Print(&right))?;
        }

        // draw bottom
        let (bottom_left, inverse) = T::bottom_left();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x, y+h-1))?.queue(Print(&bottom_left))?;

        let (bottom, inverse) = T::bottom();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x+1, y+h-1))?.queue(Print(&String::from(bottom).repeat((w-2) as usize)))?;

        let (bottom_right, inverse) = T::bottom_right();
        set_colors(out, inverse)?;
        out.queue(MoveTo(x+w-1, y+h-1))?.queue(Print(&bottom_right))?;
        
        // Draw contained element
        self.1.render(out, Area(x+1, y+1, w-2, h-2))
    });
}

/// A border character, and whether it should be rendered inverse.
pub type BorderChar = (char, bool);

/// A set of border characters.
pub trait BorderStyle {
    fn top       () -> BorderChar;
    fn top_left  () -> BorderChar;
    fn top_right () -> BorderChar;

    fn left  () -> BorderChar;
    fn right () -> BorderChar;

    fn bottom       () -> BorderChar;
    fn bottom_left  () -> BorderChar;
    fn bottom_right () -> BorderChar;
}

/// An inset border with more vertical space.
pub struct InsetTall;

impl BorderStyle for InsetTall {
    fn top       () -> BorderChar { ('▇', true)  }
    fn top_left  () -> BorderChar { ('▊', true)  }
    fn top_right () -> BorderChar { ('▎', false) }

    fn left  () -> BorderChar { ('▊', true)  }
    fn right () -> BorderChar { ('▎', false) }

    fn bottom       () -> BorderChar { ('▁', false) }
    fn bottom_left  () -> BorderChar { ('▊', true)  }
    fn bottom_right () -> BorderChar { ('▎', false) }
}

/// An inset border with more horizontal space.
pub struct InsetWide;

impl BorderStyle for InsetWide {
    fn top       () -> BorderChar { ('▁', false) }
    fn top_left  () -> BorderChar { ('▁', false) }
    fn top_right () -> BorderChar { ('▁', false) }

    fn left  () -> BorderChar { ('▎', false) }
    fn right () -> BorderChar { ('▊', true)  }

    fn bottom       () -> BorderChar { ('▇', true)  }
    fn bottom_left  () -> BorderChar { ('▇', true)  }
    fn bottom_right () -> BorderChar { ('▇', true) }
}

#[cfg(test)]
mod test {

    use thatsit::{Area, Widget};

    #[test]
    fn test_borders () {

        use crate::{Border, InsetTall, InsetWide};

        let mut output = Vec::<u8>::new();
        let layout = Border(InsetTall, "foo");
        layout.render(&mut output, Area(0, 0, 5, 5));
        panic!("{}", std::str::from_utf8(&output).unwrap());

        let mut output = Vec::<u8>::new();
        let layout = Border(InsetWide, "foo");
        layout.render(&mut output, Area(0, 0, 5, 5));
        panic!("{}", std::str::from_utf8(&output).unwrap());

    }
}
