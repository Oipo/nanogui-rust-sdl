extern crate nanovg;

#[derive(Clone, Debug)]
pub struct WidgetObj {
    parent: Option<Box<WidgetObj>>,
    children: Vec<WidgetObj>,
    //theme: theme,
    //layout: layout,
    id: String,
    pos: (u32, u32),
    pub size: (u32, u32),
    fixed_size: (u32, u32),

    pub visible: bool,
    enabled: bool,
    focused: bool,
    mouse_focus: bool,
    tooltip: String,
    font_size: i32,
    //cursor: cursor
}

pub trait Widget {
    fn parent(&self) -> Option<WidgetObj>;
    fn absolute_position(&self) -> (u32, u32);
    fn visible_recursive(&self) -> bool;
    fn contains(&self, p: (u32, u32)) -> bool;
    fn find_widget(&self, p: (u32, u32)) -> Option<WidgetObj>;
    fn draw(&self, nanovg_context: &nanovg::Context);
}

impl Widget for WidgetObj {
    fn parent(&self) -> Option<WidgetObj> {
        match self.parent.clone() {
            None => None,
            Some(box val) => Some(val)
        }
    }

    fn absolute_position(&self) -> (u32, u32) {
        match self.parent.clone() {
            None => self.pos.clone(),
            Some(box val) =>  {
                let (par_x, par_y) = val.absolute_position();
                (par_x + self.pos.0, par_y + self.pos.1)
            }
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible {
            return false
        }

        match self.parent.clone() {
            None => self.visible,
            Some(box val) => val.visible_recursive()
        }
    }

    fn contains(&self, p: (u32, u32)) -> bool {
        // TODO
        return false
    }

    fn find_widget(&self, p: (u32, u32)) -> Option<WidgetObj> {
        return None
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        for child in self.children.clone() {
            child.draw(nanovg_context);
        }
    }
}

impl WidgetObj {
    pub fn new(id: String) -> WidgetObj {
        WidgetObj {
            parent: None,
            children: Vec::new(),
            id: id,
            pos: (0, 0),
            size: (0, 0),
            fixed_size: (0, 0),
            visible: false,
            enabled: false,
            focused: false,
            mouse_focus: false,
            tooltip: String::new(),
            font_size: 12
        }
    }
}
