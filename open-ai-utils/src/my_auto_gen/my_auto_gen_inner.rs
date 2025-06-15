use std::sync::Arc;

use crate::{FunctionDescriptionJsonModel, my_auto_gen::ToolFunctionAbstract};

pub struct MyAutoGenInner {
    tool_functions: Vec<(
        &'static str,
        Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>,
    )>,
    func_json_descriptions: Vec<FunctionDescriptionJsonModel>,
}

impl MyAutoGenInner {
    pub fn new() -> Self {
        Self {
            tool_functions: Default::default(),
            func_json_descriptions: Default::default(),
        }
    }
    pub fn register(
        &mut self,
        func_name: &'static str,
        description: FunctionDescriptionJsonModel,
        tool_function: Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>,
    ) {
        self.tool_functions.push((func_name, tool_function));

        self.func_json_descriptions.push(description);
    }

    pub fn get_func_descriptions(&self) -> &[FunctionDescriptionJsonModel] {
        self.func_json_descriptions.as_slice()
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
