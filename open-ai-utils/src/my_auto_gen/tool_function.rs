use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::FunctionToolCallDescription;

#[async_trait::async_trait]
pub trait ToolFunction<ParamsType: FunctionToolCallDescription> {
    async fn callback(&self, params: ParamsType) -> String;
}

#[async_trait::async_trait]
pub trait ToolFunctionAbstract {
    async fn call(&self, func: &str, params: &str) -> String;
}

pub struct ToolFunctionHolder<ParamsType: FunctionToolCallDescription> {
    inner: Arc<dyn ToolFunction<ParamsType> + Send + Sync + 'static>,
    pub func_name: &'static str,
}

impl<ParamsType: FunctionToolCallDescription> ToolFunctionHolder<ParamsType> {
    pub fn new(
        func_name: &'static str,
        func: Arc<dyn ToolFunction<ParamsType> + Send + Sync + 'static>,
    ) -> Self {
        Self {
            func_name,
            inner: func,
        }
    }
}

#[async_trait::async_trait]
impl<ParamsType: FunctionToolCallDescription + DeserializeOwned> ToolFunctionAbstract
    for ToolFunctionHolder<ParamsType>
{
    async fn call(&self, _func: &str, params: &str) -> String {
        let data: ParamsType = serde_json::from_str(params).unwrap();
        self.inner.callback(data).await
    }
}
