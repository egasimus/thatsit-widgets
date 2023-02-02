use crate::{*, crossterm::{cursor::*,style::*}};

/// A border around another widget
#[derive(Copy, Clone, Default)]
pub struct Border<T: BorderStyle, U: BorderTheme, W: Widget>(pub T, pub U, pub W);

impl<T: BorderStyle, U: BorderTheme, W: Widget> Widget for Border<T, U, W> {
    impl_render!(self, out, area => {
        let Area(x, y, w, h) = area;
        if w == 0 || h == 0 { return Ok((0, 0)) }

        // draw top
        let (top_left, fg, bg) = T::top_left(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x, y))?.queue(Print(&top_left))?;

        let (top, fg, bg) = T::top(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x+1, y))?.queue(Print(&String::from(top).repeat((w-2) as usize)))?;

        let (top_right, fg, bg) = T::top_right(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x+w-1, y))?.queue(Print(&top_right))?;

        // draw sides and background
        let (left, fg, bg) = T::left(&self.1);
        set_colors(out, &fg, &bg)?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x, y))?.queue(Print(&left))?;
        }

        set_colors(out, &self.1.hi(), &self.1.bg())?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x+1, y))?.queue(Print(&" ".repeat((w-2) as usize)))?;
        }

        let (right, fg, bg) = T::right(&self.1);
        set_colors(out, &fg, &bg)?;
        for y in y+1..y+h-1 {
            out.queue(MoveTo(x+w-1, y))?.queue(Print(&right))?;
        }

        // draw bottom
        let (bottom_left, fg, bg) = T::bottom_left(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x, y+h-1))?.queue(Print(&bottom_left))?;

        let (bottom, fg, bg) = T::bottom(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x+1, y+h-1))?.queue(Print(&String::from(bottom).repeat((w-2) as usize)))?;

        let (bottom_right, fg, bg) = T::bottom_right(&self.1);
        set_colors(out, &fg, &bg)?;
        out.queue(MoveTo(x+w-1, y+h-1))?.queue(Print(&bottom_right))?;

        // Draw contained element
        set_colors(out, &None, &self.1.bg())?;
        self.2.render(out, Area(x+1, y+1, w-2, h-2))
    });
}

fn set_colors (out: &mut dyn Write, fg: &Option<Color>, bg: &Option<Color>) -> Result<()> {
    out.queue(ResetColor)?;
    if let Some(fg) = fg {
        out.queue(SetForegroundColor(*fg))?;
    }
    if let Some(bg) = bg {
        out.queue(SetBackgroundColor(*bg))?;
    }
    Ok(())
}

/// A set of colors to use for rendering a border.
pub trait BorderTheme {
    /// The color outside the box
    fn out (&self) -> Option<Color> { None }
    /// The background of the box
    fn bg  (&self) -> Option<Color> { None }
    /// One border color.
    fn hi  (&self) -> Option<Color>;
    /// The other border color.
    fn lo  (&self) -> Option<Color>;
}

/// Colors for an inset grey border.
pub struct Inset;

impl BorderTheme for Inset {
    fn bg (&self) -> Option<Color> {
        Some(Color::AnsiValue(235))
    }
    fn hi (&self) -> Option<Color> {
        Some(Color::AnsiValue(240))
    }
    fn lo (&self) -> Option<Color> {
        Some(Color::AnsiValue(16))
    }
}

/// Colors for an outset grey border.
pub struct Outset;

impl BorderTheme for Outset {
    fn bg (&self) -> Option<Color> {
        Some(Color::AnsiValue(235))
    }
    fn hi (&self) -> Option<Color> {
        Some(Color::AnsiValue(16))
    }
    fn lo (&self) -> Option<Color> {
        Some(Color::AnsiValue(240))
    }
}

/// A border character, and its foreground and background colors.
pub type BorderChar = (char, Option<Color>, Option<Color>);

/// A set of characters to use for rendering a border.
pub trait BorderStyle {
    fn top (theme: &impl BorderTheme) -> BorderChar;
    fn top_left (theme: &impl BorderTheme) -> BorderChar;
    fn top_right (theme: &impl BorderTheme) -> BorderChar;
    fn left (theme: &impl BorderTheme) -> BorderChar;
    fn right (theme: &impl BorderTheme) -> BorderChar;
    fn bottom (theme: &impl BorderTheme) -> BorderChar;
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar;
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar;
}

/// A border with more vertical space.
pub struct Tall;

impl BorderStyle for Tall {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.lo())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.hi(), theme.bg())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
}

/// A border with more horizontal space.
pub struct Wide;

impl BorderStyle for Wide {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.lo(), theme.bg())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.hi())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
}

/// A border with the default border characters.
pub struct Flat;

impl BorderStyle for Flat {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('─', theme.hi(), theme.bg())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('┌', theme.hi(), theme.bg())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('┐', theme.hi(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('│', theme.hi(), theme.bg())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('│', theme.hi(), theme.bg())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('─', theme.hi(), theme.bg())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('└', theme.hi(), theme.bg())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('┘', theme.hi(), theme.bg())
    }
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
