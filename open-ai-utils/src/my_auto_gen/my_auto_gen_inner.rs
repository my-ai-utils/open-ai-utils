use std::sync::Arc;

use crate::my_auto_gen::{RemoteToolFunctionsHandler, ToolFunctionAbstract, ToolFunctions};

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

    pub fn get_func(
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
}
