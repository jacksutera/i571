//! An env maintains a binding between string IDs and V Values.  It is largely
//! immutable except that existing bindings can be replaced.  Note that
//! an env clones both the id and value, so that it has ownership over
//! the cloned values.

use std::cell::RefCell;

use crate::list::{List};

//support interior mutability in a binding
type Binding<V> = (String, RefCell<V>);

#[derive(Debug, Clone)]
pub struct Env<V> {
    env: List<Binding<V>>,
    len: usize,
}

#[allow(dead_code)]
impl<V: Clone> Env<V> {

    pub fn new() -> Self {
	Env {
	    env: List::<Binding<V>>::new(),
	    len: 0,
	}
    }

    /// Add id = value binding.  Takes ownership of value.   Returns new env.
    pub fn push(&self, id: &str, value: V) -> Self {
	Env {
	    env: self.env.prepend((String::from(id), RefCell::new(value))),
	    len: self.len + 1,
	}
    }

    /// Returns value for id; the value can be owned by the caller.
    pub fn get(&self, id: &str) -> Option<V> {
	for binding in self.env.iter() {
	    if binding.0 == id {
		return Some(binding.1.borrow().clone());
	    }
	}
	return None;
    }

    /// set binding for id to value if it exists, returning previous
    /// value.  Takes ownership of value, while returned value can
    /// be owned by the caller.
    pub fn set(&self, id: &str, value: V) -> Option<V> {
	for binding in self.env.iter() {
	    if binding.0 == id {
		return Some(binding.1.replace(value));
	    }
	}
	return None;
    }

    pub fn len(&self) -> usize { self.len }

    
}

#[cfg(test)]
mod test {
    use super::Env;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Value {
	A(i32), B(&'static str),
    }

    const VAL1: Value = Value::A(42);
    const VAL2: Value = Value::B("hello");
    const VAL3: Value = Value::A(99);
    const VAL4: Value = Value::B("world");

    #[test]
    fn test_push_get() {
	let env1 = Env::<Value>::new();
	let env2 = env1.push("a", VAL1);
	let env3 = env2.push("a", VAL2);
	let env4 = env2.push("b", VAL3);

	assert_eq!(env1.len(), 0);
	assert_eq!(env2.len(), 1);
	assert_eq!(env3.len(), 2);
	
	assert_eq!(env1.get("a"), None);
	assert_eq!(env2.get("a"), Some(VAL1));
	assert_eq!(env3.get("a"), Some(VAL2));
	assert_eq!(env3.get("b"), None);
	assert_eq!(env4.get("b"), Some(VAL3));
	
    }

}
