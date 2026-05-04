//! Execution State: Current Runtime State of the System
//!
//! Tracks the execution stack, variable bindings, and current context
//! that forms the basis for external probing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime value in the execution state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::None => write!(f, "null"),
        }
    }
}

/// A stack frame in the execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub name: String,
    pub variables: HashMap<String, Value>,
}

/// Complete execution state of the system
pub struct ExecutionState {
    /// Call stack of frames
    stack: Vec<StackFrame>,

    /// Global variable scope
    globals: HashMap<String, Value>,

    /// Current frame depth (for error context)
    current_frame: usize,

    /// Total instruction count
    instructions_executed: u64,

    /// Current line/context
    current_context: String,
}

impl ExecutionState {
    /// Create a new execution state
    pub fn new() -> Self {
        ExecutionState {
            stack: vec![StackFrame {
                name: "global".to_string(),
                variables: HashMap::new(),
            }],
            globals: HashMap::new(),
            current_frame: 0,
            instructions_executed: 0,
            current_context: "initialization".to_string(),
        }
    }

    /// Push a new stack frame
    pub fn push_frame(&mut self, name: String) {
        self.stack.push(StackFrame {
            name,
            variables: HashMap::new(),
        });
        self.current_frame = self.stack.len() - 1;
    }

    /// Pop the current stack frame
    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        if self.stack.len() > 1 {
            self.current_frame = self.stack.len() - 2;
            self.stack.pop()
        } else {
            None
        }
    }

    /// Set a variable in the current frame
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(frame) = self.stack.get_mut(self.current_frame) {
            frame.variables.insert(name, value);
        }
    }

    /// Get a variable from the current frame or globals
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        if let Some(frame) = self.stack.get(self.current_frame) {
            if let Some(val) = frame.variables.get(name) {
                return Some(val.clone());
            }
        }
        self.globals.get(name).cloned()
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }

    /// Record an instruction execution
    pub fn record_instruction(&mut self, context: String) {
        self.instructions_executed += 1;
        self.current_context = context;
    }

    /// Get the current call stack as a string
    pub fn stack_trace(&self) -> String {
        let mut trace = String::from("Call Stack:\n");
        for (i, frame) in self.stack.iter().enumerate() {
            trace.push_str(&format!("  [{}] {}\n", i, frame.name));
        }
        trace
    }

    /// Get a snapshot of current state
    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            frame_depth: self.stack.len(),
            current_frame_name: self
                .stack
                .get(self.current_frame)
                .map(|f| f.name.clone())
                .unwrap_or_default(),
            variables_count: self
                .stack
                .get(self.current_frame)
                .map(|f| f.variables.len())
                .unwrap_or(0),
            globals_count: self.globals.len(),
            instructions_executed: self.instructions_executed,
            context: self.current_context.clone(),
        }
    }

    /// Get all variables in current scope (including globals)
    pub fn all_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        // Add globals
        for (k, v) in &self.globals {
            vars.insert(k.clone(), v.to_string());
        }

        // Add current frame variables (override globals)
        if let Some(frame) = self.stack.get(self.current_frame) {
            for (k, v) in &frame.variables {
                vars.insert(k.clone(), v.to_string());
            }
        }

        vars
    }

    /// Get depth of the call stack
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Instructions executed so far
    pub fn instruction_count(&self) -> u64 {
        self.instructions_executed
    }
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self::new()
    }
}

/// A snapshot of execution state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub frame_depth: usize,
    pub current_frame_name: String,
    pub variables_count: usize,
    pub globals_count: usize,
    pub instructions_executed: u64,
    pub context: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_binding() {
        let mut state = ExecutionState::new();
        state.set_variable("x".to_string(), Value::Integer(42));

        let val = state.get_variable("x");
        assert_eq!(val, Some(Value::Integer(42)));
    }

    #[test]
    fn test_stack_frames() {
        let mut state = ExecutionState::new();
        state.push_frame("function1".to_string());

        state.set_variable("local".to_string(), Value::String("test".to_string()));
        assert_eq!(state.stack_depth(), 2);

        state.pop_frame();
        assert_eq!(state.stack_depth(), 1);
    }

    #[test]
    fn test_snapshot() {
        let mut state = ExecutionState::new();
        state.set_global("global_var".to_string(), Value::Boolean(true));
        state.push_frame("func".to_string());
        state.set_variable("local_var".to_string(), Value::Float(3.14));

        let snap = state.snapshot();
        assert_eq!(snap.frame_depth, 2);
        assert_eq!(snap.globals_count, 1);
        assert_eq!(snap.variables_count, 1);
    }
}
