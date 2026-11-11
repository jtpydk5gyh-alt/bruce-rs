//! Hierarchical menu and navigation, hardware-agnostic and host-tested. A tree of
//! [`MenuNode`]s; [`Navigator`] tracks the current level and cursor and turns button
//! presses into [`MenuEvent`]s. Selecting a submenu descends, Back climbs.

use alloc::vec::Vec;
use bruce_hal::{Button, Display};

/// A menu entry: a leaf (an action) or a submenu with children.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuNode {
    pub label: &'static str,
    pub children: Vec<MenuNode>,
}

impl MenuNode {
    pub fn leaf(label: &'static str) -> Self {
        Self {
            label,
            children: Vec::new(),
        }
    }

    pub fn submenu(label: &'static str, children: Vec<MenuNode>) -> Self {
        Self { label, children }
    }

    pub fn is_submenu(&self) -> bool {
        !self.children.is_empty()
    }
}

/// What a button press did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEvent {
    Moved,
    Entered(&'static str),
    Back,
    Selected(&'static str),
    None,
}

pub struct Navigator {
    root: Vec<MenuNode>,
    path: Vec<usize>,
    cursor: usize,
}

impl Navigator {
    pub fn new(root: Vec<MenuNode>) -> Self {
        Self {
            root,
            path: Vec::new(),
            cursor: 0,
        }
    }

    /// The items at the current level (walks the tree down the entered path).
    pub fn items(&self) -> &[MenuNode] {
        let mut items = self.root.as_slice();
        for &index in &self.path {
            items = items[index].children.as_slice();
        }
        items
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn depth(&self) -> usize {
        self.path.len()
    }

    pub fn on_button(&mut self, button: Button) -> MenuEvent {
        let len = self.items().len();
        if len == 0 {
            return MenuEvent::None;
        }
        match button {
            Button::Up => {
                self.cursor = self.cursor.checked_sub(1).unwrap_or(len - 1);
                MenuEvent::Moved
            }
            Button::Down => {
                self.cursor = (self.cursor + 1) % len;
                MenuEvent::Moved
            }
            Button::Select => {
                let node = &self.items()[self.cursor];
                let label = node.label;
                if node.is_submenu() {
                    self.path.push(self.cursor);
                    self.cursor = 0;
                    MenuEvent::Entered(label)
                } else {
                    MenuEvent::Selected(label)
                }
            }
            Button::Back => match self.path.pop() {
                Some(parent_cursor) => {
                    self.cursor = parent_cursor;
                    MenuEvent::Back
                }
                None => MenuEvent::None,
            },
        }
    }

    pub fn render(&self, display: &mut impl Display) {
        display.clear();
        for (i, node) in self.items().iter().enumerate() {
            let cursor = if i == self.cursor { ">" } else { " " };
            let tail = if node.is_submenu() { " >" } else { "" };
            display.print(0, i as u16, &format!("{cursor} {}{tail}", node.label));
        }
        display.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> Vec<MenuNode> {
        vec![
            MenuNode::submenu("RF", vec![MenuNode::leaf("Scan"), MenuNode::leaf("Replay")]),
            MenuNode::leaf("About"),
        ]
    }

    #[test]
    fn cursor_wraps_both_ends() {
        let mut nav = Navigator::new(tree());
        nav.on_button(Button::Up);
        assert_eq!(nav.cursor(), 1);
        nav.on_button(Button::Down);
        assert_eq!(nav.cursor(), 0);
    }

    #[test]
    fn enters_and_leaves_submenu() {
        let mut nav = Navigator::new(tree());
        assert_eq!(nav.on_button(Button::Select), MenuEvent::Entered("RF"));
        assert_eq!(nav.depth(), 1);
        assert_eq!(nav.items().len(), 2);
        assert_eq!(nav.items()[0].label, "Scan");

        assert_eq!(nav.on_button(Button::Back), MenuEvent::Back);
        assert_eq!(nav.depth(), 0);
        // Cursor is restored to the submenu we came from.
        assert_eq!(nav.cursor(), 0);
    }

    #[test]
    fn selecting_a_leaf_reports_it() {
        let mut nav = Navigator::new(tree());
        nav.on_button(Button::Down); // move to "About"
        assert_eq!(nav.on_button(Button::Select), MenuEvent::Selected("About"));
        assert_eq!(nav.depth(), 0); // leaves don't descend
    }

    #[test]
    fn back_at_root_is_noop() {
        let mut nav = Navigator::new(tree());
        assert_eq!(nav.on_button(Button::Back), MenuEvent::None);
        assert_eq!(nav.depth(), 0);
    }
}
