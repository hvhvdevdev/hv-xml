#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use crate::ReaderState::{LookingForClosingTagClose, LookingForRootOpen, LookingForRootClose, LookingForClosingTagOpen, LookingForClosingTagSlash};

#[derive(Debug)]
pub struct Attribute<'a> {
    pub name: &'a str,
    pub str_value: &'a str,
}

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub body: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

enum ReaderState {
    LookingForRootOpen,
    LookingForRootClose,
    LookingForClosingTagOpen,
    LookingForClosingTagSlash,
    LookingForClosingTagClose,
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

impl<'a> Node<'a> {
    pub fn read(source: &'a str) -> Option<Node<'a>> {
        let mut root_open_pos: Option<usize> = None;
        let mut root_close_pos: Option<usize> = None;
        let mut state = ReaderState::LookingForRootOpen;

        // Find the location of Root opening element.
        for i in 0..source.len() {
            match state {
                ReaderState::LookingForRootOpen => if source.chars().nth(i).unwrap() == '<' {
                    root_open_pos = Some(i);
                    state = LookingForRootClose;
                }
                ReaderState::LookingForRootClose => if source.chars().nth(i).unwrap() == '>' {
                    root_close_pos = Some(i);
                    state = LookingForClosingTagClose;
                    break;
                }
                _ => return None
            }
        }
        // Find the location of the closing element.
        let mut closing_open_pos: Option<usize> = None;
        let mut closing_close_pos: Option<usize> = None;
        let mut closing_slash_pos: Option<usize> = None;

        for i in (0..source.len()).rev() {
            match state {
                ReaderState::LookingForClosingTagClose => if source.chars().nth(i).unwrap() == '>' {
                    closing_close_pos = Some(i);
                    state = LookingForClosingTagSlash
                }
                ReaderState::LookingForClosingTagSlash => if source.chars().nth(i).unwrap() == '/' {
                    closing_slash_pos = Some(i);
                    state = LookingForClosingTagOpen
                }
                ReaderState::LookingForClosingTagOpen => if source.chars().nth(i).unwrap() == '<' {
                    closing_open_pos = Some(i);
                    break;
                }
                _ => return None
            }
        }

        match (root_open_pos, root_close_pos, closing_open_pos, closing_slash_pos, closing_close_pos) {
            // Different opening and closing tags...
            (Some(root_open_pos), Some(root_close_pos), Some(_), Some(closing_slash_pos), Some(closing_close_pos))
            if &source[root_open_pos + 1..root_close_pos] != &source[closing_slash_pos + 1..closing_close_pos] => None,
            // Okay.
            (Some(root_open_pos), Some(root_close_pos), Some(closing_open_pos), Some(_), Some(_)) =>
                Some(Node
                {
                    name: &source[root_open_pos + 1..root_close_pos],
                    body: &source[root_close_pos + 1..closing_open_pos],
                    attributes: vec![],
                }),
            // Couldn't get the positions?
            (_, _, _, _, _) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn simple() {
        let n = Node::read("<xml></xml>");
        assert_eq!(n.unwrap().name, "xml")
    }

    #[test]
    fn with_body() {
        let n = Node::read("<xml>Ahihi</xml>");
        assert_eq!(n.unwrap().name, "xml")
    }

    #[test]
    fn with_hard_body() {
        let n = Node::read("<xml><xml></xml>");
        assert_eq!(n.is_none(), true)
    }


    #[test]
    fn wrong_closing() {
        let n = Node::read("<xml></Xml>");
        assert_eq!(n.is_none(), true)
    }

    #[test]
    fn no_slash() {
        let n = Node::read("<xml><xml>");
        assert_eq!(n.is_none(), true)
    }

    #[test]
    fn no_open() {
        let n = Node::read("xml><xml>");
        assert_eq!(n.is_none(), true)
    }
}
