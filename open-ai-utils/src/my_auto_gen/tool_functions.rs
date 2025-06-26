use std::sync::Arc;

use crate::{ToolsDescriptionJsonModel, my_auto_gen::ToolFunctionAbstract};

pub struct ToolFunctions {
    tool_functions: Vec<(
        &'static str,
        Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>,
    )>,
    func_json_descriptions: Vec<serde_json::Value>,
}

impl ToolFunctions {
    pub fn new() -> Self {
        Self {
            tool_functions: Default::default(),
            func_json_descriptions: Default::default(),
        }
    }
    pub fn register(
        &mut self,
        func_name: &'static str,
        description: serde_json::Value,
        tool_function: Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>,
    ) {
        self.tool_functions.push((func_name, tool_function));

        self.func_json_descriptions.push(description);
    }

    pub fn get_tools_description(&self) -> serde_json::Value {
        let mut tools = Vec::new();

        for func_description in self.func_json_descriptions.iter() {
            tools.push(ToolsDescriptionJsonModel {
                tp: "function".to_string(),
                function: Some(func_description.clone()),
            });
        }

        serde_json::to_value(tools).unwrap()
    }

    pub fn get_func(
        &self,
        func_name: &str,
    ) -> Option<Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>> {
        for itm in &self.tool_functions {
            if itm.0 == func_name {
                return Some(itm.1.clone());
            }
        }
        None
    }
}
