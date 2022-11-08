use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolScope {
    Global,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: &str) -> &Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Global,
            index: self.num_definitions,
        };

        self.store.insert(name.to_string(), symbol);
        self.num_definitions += 1;
        return self.store.get(name).unwrap();
    }

    pub fn resolve(&mut self, name: &str) -> Option<&Symbol> {
        self.store.get(name)
    }
}
