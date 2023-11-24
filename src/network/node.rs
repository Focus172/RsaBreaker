use std::{
    cell::RefCell,
    rc::Rc,
    sync::{atomic::{AtomicU32, Ordering}, Arc},
};

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// A floating point number that is the weight from the bias node
    pub bias: f32,
    /// An array of values of length equal to previous layers number of nodes that
    /// repersents how much the output of that node should be multiplied by before
    /// being added to this one.
    pub weights: Box<[f32]>,
}

#[derive(Debug, Clone)]
pub struct RefNode {
    inner: Rc<RefCell<Node>>,
    /// repersents the output that the node emited this or last cycle
    poutput: Rc<AtomicU32>, // bytes of f32
    /// a calculated value based on the error from last cycle
    err_sig: Rc<AtomicU32>, // bytes of f32
}

impl RefNode {
    pub fn get(&self) -> std::cell::Ref<'_, Node> {
        self.inner.borrow()
    }

    pub fn get_mut(&self) -> std::cell::RefMut<'_, Node> {
        self.inner.borrow_mut()
    }

    pub fn try_get_mut(&mut self) -> Option<&mut Node> {
        Rc::get_mut(&mut self.inner).map(RefCell::get_mut)
    }

    #[inline]
    pub fn output(&self) -> f32 {
        let output = self.poutput.load(Ordering::Relaxed);
        f32::from_bits(output)
    }

    pub fn set_output(&self, output: f32) {
        self.poutput.store(output.to_bits(), Ordering::Relaxed);
    }

    #[inline]
    pub fn error_signal(&self) -> f32 {
        let output = self.err_sig.load(Ordering::Relaxed);
        f32::from_bits(output)
    }

    pub fn set_error_signal(&self, error_signal: f32) {
        self.err_sig
            .store(error_signal.to_bits(), Ordering::Relaxed);
    }

    #[inline]
    pub fn output_derivative(&self) -> f32 {
        let output = self.output();
        output * (1. - output)
    }
}

impl Node {
    pub fn new(width: usize) -> Self {
        Node {
            bias: 0.3,
            weights: util::rand::weights(width, 0.3, 0.7),
        }
    }
}

impl From<Node> for RefNode {
    fn from(value: Node) -> Self {
        let poutput = f32::to_bits(0.0);
        let err_sig = f32::to_bits(0.0);
        Self {
            inner: Rc::new(RefCell::new(value)),
            poutput: Rc::new(AtomicU32::new(poutput)),
            err_sig: Rc::new(AtomicU32::new(err_sig)),
        }
    }
}

impl From<&RefNode> for Node {
    /// Copies the data out of the node
    fn from(value: &RefNode) -> Self {
        value.inner.borrow().clone()
    }
}
