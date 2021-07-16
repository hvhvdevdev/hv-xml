#![no_std]

use core::slice::Iter;
use core::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Node<'a> {
    pub name: &'a str,
    pub body: &'a str,
    after: &'a str,
    opener: &'a str,
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn count_until(s: &str, c: char) -> Option<usize> {
    if s.len() == 0 {
        return None;
    }

    let mut n: usize = 0;
    Some(
        if s.chars().nth(0).unwrap() == c {
            0
        } else {
            if
            let Some(x) = count_until(&s[1..], c)
            {
                1 + x
            } else {
                return None;
            }
        }
    )
}

fn get_tag_pos(s: &str) -> Option<(usize, usize)> {
    if let Some(start) = count_until(s, '<') {
        if let Some(end) = count_until(&s[start..], '>') {
            Some((start + 1, end + start))
        } else { None }
    } else { None }
}

fn get_tag_name(s: &str, pos: (usize, usize)) -> &str {
    let start = if s.chars().nth(pos.0).unwrap() == '/' {
        1
    } else {
        0
    };

    let end = count_until(s, ' ')
        .unwrap_or(0);

    &s[start + pos.0..end + pos.1]
}

impl<'a> Node<'a> {
    pub fn read(source: &'a str) -> Option<Node<'a>> {
        let opener_pos = if let Some(pos) = get_tag_pos(source) {
            pos
        } else {
            return None;
        };

        let opener_name = get_tag_name(source, opener_pos);
        let mut nest = 0usize;
        let mut current_pos = opener_pos.1;

        loop {
            let pos = if let Some(p) = get_tag_pos(&source[current_pos..]) {
                (p.0 + current_pos, p.1 + current_pos)
            } else {
                return None;
            };

            if get_tag_name(source, pos) == opener_name {
                if source.chars().nth(pos.0).unwrap() == '/' {
                    if nest == 0 {
                        return Some(Node { after: &source[pos.1..], opener: &source[opener_pos.0..opener_pos.1], body: &source[opener_pos.1..pos.0], name: opener_name });
                    } else {
                        nest = nest - 1;
                    }
                } else {
                    nest = nest + 1;
                }
            }

            current_pos = pos.1;
        }
    }
}

impl<'a> Iterator for Node<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Node::read(self.after)
    }
}

impl<'a> Index<&str> for Node<'a> {
    type Output = Option<&'a str>;

    fn index(&self, index: &str) -> &Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn count_until() {
        assert_eq!(crate::count_until("abcd<d>", '<'), Some(4));
        assert_eq!(crate::count_until("abcd<d>", 'a'), Some(0));
        assert!(crate::count_until("abc", 'd').is_none());
    }

    #[test]
    fn get_tag() {
        assert_eq!(crate::get_tag_pos(" <Hello>"), Some((2, 7)));
        assert_eq!(crate::get_tag_pos(" </Hello>"), Some((2, 8)));
    }

    #[test]
    fn get_tag_name() {
        let s1 = "   <xml v=1>";
        let s2 = "   </xml>";
        let pos1 = crate::get_tag_pos(s1).unwrap();
        let pos2 = crate::get_tag_pos(s2).unwrap();
        assert_eq!(crate::get_tag_name(s1, pos1), "xml");
        assert_eq!(crate::get_tag_name(s2, pos2), "xml");
    }

    #[test]
    fn simple() {
        let n = Node::read("<xml></xml>");
        assert_eq!(n.unwrap().name, "xml")
    }

    #[test]
    fn simple_salt() {
        let n = Node::read("aaa<xml></xml>");
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
        assert!(n.is_none())
    }


    #[test]
    fn wrong_closing() {
        let n = Node::read("<xml></Xml>");
        assert!(n.is_none())
    }

    #[test]
    fn no_slash() {
        let n = Node::read("<xml><xml>");
        assert!(n.is_none())
    }

    #[test]
    fn no_open() {
        let n = Node::read("xml><xml>");
        assert!(n.is_none())
    }

    #[test]
    fn only_close() {
        let n = Node::read("<xml>");
        assert!(n.is_none())
    }

    #[test]
    fn sister() {
        let n = Node::read("<alice>Alice</alice><maria></maria>");
        let n2 = n.unwrap().next();
        assert_eq!(n2.unwrap().name, "maria");
    }

    #[test]
    fn sister_daughter() {
        let n = Node::read("<mama><alice>Alice</alice><maria></maria></mama>");
        assert_eq!(n.unwrap().name, "mama");
        let n = Node::read(n.unwrap().body);
        assert_eq!(n.unwrap().name, "alice");
        let n = n.unwrap().next();
        assert_eq!(n.unwrap().name, "maria");
    }

    #[test]
    fn nest() {
        let n = Node::read("   <Foo name=foo><Foo></Foo></Foo>  ");
        assert_eq!(n.unwrap().name, "Foo");
        let n = Node::read(n.unwrap().body);
        assert_eq!(n.unwrap().name, "Foo");
        let n = Node::read(n.unwrap().body);
        assert!(n.is_none());
    }

    #[test]
    fn attr() {
        let n = Node::read("   <Foo name=foo></Foo>  ");
        assert_eq!(n.unwrap()["name"].unwrap(), "foo");
    }
}
