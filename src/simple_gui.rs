pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(self.label.as_str()).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str("| ").unwrap();
        self.label.draw_into(buffer);
        buffer.write_str(" |").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut wrapper = "=".repeat(self.width());
        wrapper.push('\n');

        buffer.write_str(&wrapper).unwrap();
        buffer.write_str(self.title.as_str()).unwrap();
        buffer.write_str("\n").unwrap();
        buffer.write_str(&wrapper).unwrap();

        for widget in &self.widgets {
            buffer.write_str("\n").unwrap();
            widget.draw_into(buffer);
            buffer.write_str("\n").unwrap();
        }
    }
}

#[test]
fn test_behavior() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));

    window.draw();
}
