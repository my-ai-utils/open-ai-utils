use std::sync::Arc;

use crate::my_auto_gen::ToolFunctionAbstract;

#[async_trait::async_trait]
pub trait RemoteToolFunctions {
    async fn get_tools_description(&self) -> String;
    async fn tool_call(&self, fn_name: &str, params: &str) -> String;
}

pub struct RemoteToolFunctionsHandler {
    pub data_src: Arc<dyn RemoteToolFunctions + Send + Sync + 'static>,
}

#[async_trait::async_trait]
impl ToolFunctionAbstract for RemoteToolFunctionsHandler {
    async fn call(&self, func_name: &str, params: &str) -> String {
        self.data_src.tool_call(func_name, params).await
    }
}
