use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::FunctionToolCallDescription;

#[async_trait::async_trait]
pub trait ToolFunction<ParamsType: FunctionToolCallDescription> {
    async fn callback(&self, params: ParamsType, ctx: &str) -> Result<String, String>;
}

#[async_trait::async_trait]
pub trait ToolFunctionAbstract {
    async fn call(&self, func: &str, params: &str, ctx: &str) -> Result<String, String>;
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
impl<ParamsType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static>
    ToolFunctionAbstract for ToolFunctionHolder<ParamsType>
{
    async fn call(&self, fn_name: &str, params: &str, ctx: &str) -> Result<String, String> {
        let data: Result<ParamsType, _> = serde_json::from_str(params);
        match data {
            Ok(data) => self.inner.callback(data, ctx).await,
            Err(err) => Err(format!(
                "Can not deserialize parameters for fn {}. Err: {}",
                fn_name, err
            )),
        }
    }
}
