use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::{
    FunctionDescriptionJsonModel, FunctionToolCallDescription, ToolsDescriptionJsonModel,
    my_auto_gen::{ToolFunction, ToolFunctionAbstract, ToolFunctionHolder},
};

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

    pub fn register_function<
        ParamType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static,
        TToolFunction: ToolFunction<ParamType> + Send + Sync + 'static,
    >(
        &mut self,
        func_name: &'static str,
        func_description: &'static str,
        tool_function: Arc<TToolFunction>,
    ) {
        let func_json_description = FunctionDescriptionJsonModel {
            name: func_name.to_string(),
            description: func_description.to_string(),
            parameters: ParamType::get_description(),
            strict: None,
        };

        let holder = ToolFunctionHolder::new(func_name, tool_function);

        let holder = Arc::new(holder);

        self.register(
            func_name,
            serde_json::to_value(func_json_description).unwrap(),
            holder,
        );
    }

    fn register(
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

    pub async fn invoke_function(
        &self,
        func_name: &str,
        params: &str,
        ctx: &str,
    ) -> Result<String, String> {
        let func = self.get_func(func_name);

        let Some(func) = func else {
            return Err(format!(
                "Function call with name {} is not found",
                func_name
            ));
        };

        func.call(func_name, params, ctx).await
    }
}
