use node::{FormalParameter, FormalParameters, Node};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FreeVariableFinder {
    pub varmap: Vec<HashSet<String>>,
    pub cur_fv: HashSet<String>,
}

impl FreeVariableFinder {
    pub fn new() -> FreeVariableFinder {
        let mut varmap = HashSet::new();
        varmap.insert("console".to_string());
        FreeVariableFinder {
            varmap: vec![varmap],
            cur_fv: HashSet::new(),
        }
    }

    pub fn run(&mut self, node: &mut Node) {
        match node {
            &mut Node::StatementList(ref mut nodes) => {
                for node in nodes {
                    self.run(node)
                }
            }
            &mut Node::FunctionDecl(ref name, ref mut fv, ref params, ref mut body) => {
                self.varmap.push(HashSet::new());
                if let &Some(ref name) = name {
                    self.varmap.last_mut().unwrap().insert(name.clone());
                }

                for param in params.clone() {
                    self.varmap.last_mut().unwrap().insert(param.name);
                }

                self.run(body);

                for v in self.varmap.last().unwrap() {
                    self.cur_fv.remove(v);
                }

                *fv = self.cur_fv.clone();

                self.varmap.pop();
                if let &Some(ref name) = name {
                    self.varmap.last_mut().unwrap().insert(name.clone());
                }
            }
            &mut Node::Call(ref mut callee, ref mut args) => {
                self.run(callee);
                for arg in args {
                    self.run(arg)
                }
            }
            &mut Node::VarDecl(ref name, ref mut init) => {
                self.varmap.last_mut().unwrap().insert(name.clone());
                if let &mut Some(ref mut init) = init {
                    self.run(init)
                }
            }
            &mut Node::Return(ref mut val) => {
                if let &mut Some(ref mut val) = val {
                    self.run(&mut **val)
                }
            }
            &mut Node::Member(ref mut parent, _) => {
                self.run(&mut *parent);
            }
            &mut Node::Identifier(ref name) => {
                if !self.varmap[0].contains(name.as_str())
                    && !self.varmap.last().unwrap().contains(name.as_str())
                {
                    self.cur_fv.insert(name.clone());
                }
            }
            &mut Node::If(ref mut cond, ref mut then, ref mut else_) => {
                self.run(&mut *cond);
                self.run(&mut *then);
                self.run(&mut *else_);
            }
            &mut Node::While(ref mut cond, ref mut body) => {
                self.run(&mut *cond);
                self.run(&mut *body);
            }
            &mut Node::Assign(ref mut dst, ref mut src) => {
                self.run(&mut *dst);
                self.run(&mut *src);
            }
            &mut Node::UnaryOp(ref mut expr, _) => {
                self.run(&mut *expr);
            }
            &mut Node::BinaryOp(ref mut lhs, ref mut rhs, _) => {
                self.run(&mut *lhs);
                self.run(&mut *rhs);
            }
            &mut Node::TernaryOp(ref mut cond, ref mut then, ref mut else_) => {
                self.run(&mut *cond);
                self.run(&mut *then);
                self.run(&mut *else_);
            }
            _ => {}
        }
    }
}