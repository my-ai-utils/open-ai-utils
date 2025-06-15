use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::{
    FunctionDescriptionJsonModel, FunctionToolCallDescription,
    my_auto_gen::{MyAutoGenInner, ToolFunction, ToolFunctionHolder},
};

pub struct MyAutoGen {
    inner: Arc<MyAutoGenInner>,
}

impl MyAutoGen {
    pub async fn register_function<
        ParamType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static,
        TToolFunction: ToolFunction<ParamType> + Send + Sync + 'static,
    >(
        &self,
        func_name: &'static str,
        func_description: &'static str,
        tool_function: Arc<TToolFunction>,
    ) {
        let func_json_description = FunctionDescriptionJsonModel {
            name: func_name.to_string(),
            description: func_description.to_string(),
            parameters: ParamType::get_description(),
        };

        let holder = ToolFunctionHolder::new(func_name, func_json_description, tool_function);

        let holder = Arc::new(holder);

        self.inner.register(func_name, holder).await;
    }

    pub async fn execute_callback(&self, func_name: &str, params: &str) -> String {
        self.inner.execute(func_name, params).await
    }
}
