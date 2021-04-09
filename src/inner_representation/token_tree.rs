use core::fmt;
use std::rc::Rc;

use crate::inner_representation::token::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenTree {
    value: Rc<Token>,
    children: Vec<TokenTree>,
}

impl TokenTree {
    pub fn new(root: Token) -> Self {
        TokenTree {
            value: Rc::new(root),
            children: vec![],
        }
    }

    pub fn add_child(&mut self, value: TokenTree) {
        self.children.push(value);
    }

    pub fn get_val(&self) -> &Token {
        &self.value
    }
}

#[derive(Debug)]
pub struct TreeBuilder {
    tree: Vec<Vec<usize>>,
    context: Vec<usize>,
    values: Vec<Token>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        TreeBuilder {
            tree: vec![],
            context: vec![],
            values: vec![],
        }
    }

    pub fn get(&self, index: usize) -> &Token {
        &self.values[self.context[self.context.len() - index]]
    }

    pub fn push(&mut self, value: Token) {
        let id = self.values.len();
        match self.context.last() {
            Some(last) => self.tree[*last].push(id),
            None => (),
        }
        self.tree.push(vec![]);
        self.context.push(id); 
        self.values.push(value);
    }

    pub fn pop(&mut self) {
        if self.context.len() > 1 {
            self.context.pop();
        }
    }

    pub fn push_one(&mut self, value: Token) {
        self.push(value);
        self.pop();
    }

    pub fn pipe_push(mut self, value: Token) -> Self {
        self.push(value);
        self
    }

    pub fn pipe_pop(mut self) -> Self {
        self.pop();
        self
    }

    pub fn pipe_push_one(mut self, value: Token) -> Self {
        self
            .pipe_push(value)
            .pipe_pop()
    }

    fn partial_build(&self, id: usize) -> TokenTree {
        let mut ans = TokenTree::new(self.values[id].clone());
        for new_id in &self.tree[id] {
            ans.add_child(self.partial_build(*new_id));
        }
        ans
    }

    pub fn build(self) -> TokenTree {
        self.partial_build(self.context[0])
    }


    fn print(&self, fmt: &mut std::fmt::Formatter, node: usize, prefix: String, last: bool) -> std::fmt::Result {
        let prefix_current = if last { "`- " } else { "|- " };

        write!(fmt, "{}{}{}\n", prefix, prefix_current, self.values[node].clone().get_text())?;

        let prefix_child = if last { "   " } else { "|  " };
        let prefix = prefix + prefix_child;

        if !self.tree[node].is_empty() {
            let last_child = self.tree[node].len() - 1;
            for (i, next) in self.tree[node].iter().enumerate() {
                self.print(fmt, *next, prefix.to_string(), i == last_child)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for TreeBuilder {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.print(fmt, self.context[0], "".to_string(), true)
    }
}
