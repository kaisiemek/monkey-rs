use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolScope {
    Global,
    Local,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
    pub outer: Option<Rc<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            num_definitions: 0,
            outer: None,
        }
    }

    pub fn with_enclosed(outer: Rc<SymbolTable>) -> Self {
        SymbolTable {
            store: HashMap::new(),
            num_definitions: 0,
            outer: Some(outer),
        }
    }

    pub fn define(&mut self, name: &str) -> &Symbol {
        let scope = match &self.outer {
            Some(_) => SymbolScope::Local,
            None => SymbolScope::Global,
        };

        let symbol = Symbol {
            name: name.to_string(),
            scope,
            index: self.num_definitions,
        };

        self.store.insert(name.to_string(), symbol);
        self.num_definitions += 1;
        return self.store.get(name).unwrap();
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.store.get(name) {
            return Some(symbol);
        }

        match &self.outer {
            Some(table) => table.resolve(name),
            None => None,
        }
    }
}
