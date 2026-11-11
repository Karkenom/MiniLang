use std::collections::HashMap;

use crate::value::Value;

pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(existing) = self.variables.get_mut(name) {
            *existing = value;
            Ok(())
        } else {
            Err(format!(
                "Нельзя присвоить значение необъявленной переменной '{}'",
                name
            ))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| {
                format!("Переменная '{}' не объявлена", name)
            })
    }
}
