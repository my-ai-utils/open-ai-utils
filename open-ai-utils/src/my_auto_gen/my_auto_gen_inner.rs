use std::sync::Arc;

use crate::{
    OpenAiRequestBodyBuilder,
    my_auto_gen::{RemoteToolFunctionsHandler, ToolFunctions},
};

pub enum MyAutoGenInner {
    NotInitialized,
    LocalToolFunctions(ToolFunctions),
    RemoteToolFunctions(Arc<RemoteToolFunctionsHandler>),
}

impl MyAutoGenInner {
    pub fn new() -> Self {
        Self::NotInitialized
    }

    pub fn unwrap_as_local_functions_mut(&mut self) -> &mut ToolFunctions {
        match self {
            MyAutoGenInner::NotInitialized => {
                panic!("Not Initialized");
            }
            MyAutoGenInner::LocalToolFunctions(local_tool_functions) => local_tool_functions,
            MyAutoGenInner::RemoteToolFunctions(_) => {
                panic!("Remote Tool Functions");
            }
        }
    }

    pub async fn populate_request_builder(&self, rb: &mut OpenAiRequestBodyBuilder) {
        match &self {
            MyAutoGenInner::NotInitialized => {}
            MyAutoGenInner::LocalToolFunctions(local_tool_functions) => {
                let tools = local_tool_functions.get_tools_description();
                rb.add_tools(tools);
            }
            MyAutoGenInner::RemoteToolFunctions(handler) => {
                let description = handler.data_src.get_tools_description().await;
                let tools: Result<serde_json::Value, _> = serde_json::from_str(&description);

                if let Err(err) = &tools {
                    println!("Can not parse tools description. Err:{}", err);
                    println!("{}", &description);
                    panic!("{}", err);
                }
                rb.add_tools(tools.unwrap());
            }
        }
    }

    /*
    fn get_func(
        &self,
        func_name: &str,
    ) -> Option<Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>> {
        match self {
            MyAutoGenInner::NotInitialized => {
                let msg = format!(
                    "Somehow there is a tool call, which asks for a func '{}'. AutoGen is in not initialized state",
                    func_name
                );
                println!("{}", msg);
                panic!("{}", msg);
            }
            MyAutoGenInner::LocalToolFunctions(local_tool_functions) => {
                local_tool_functions.get_func(func_name)
            }
            MyAutoGenInner::RemoteToolFunctions(remote_tool_functions) => {
                Some(remote_tool_functions.clone())
            }
        }
    }
    */

    pub async fn invoke_func(
        &self,
        fn_name: &str,
        params: &str,
        ctx: &str,
    ) -> Result<String, String> {
        match self {
            MyAutoGenInner::NotInitialized => {
                panic!(
                    "AutoGen does is in NotInitialized mode. fn_name:{}",
                    fn_name
                );
            }
            MyAutoGenInner::LocalToolFunctions(tool_functions) => {
                tool_functions.invoke_function(fn_name, params, ctx).await
            }
            MyAutoGenInner::RemoteToolFunctions(remote_tool_functions_handler) => {
                remote_tool_functions_handler
                    .invoke_function(fn_name, params, ctx)
                    .await
            }
        }
    }
}
