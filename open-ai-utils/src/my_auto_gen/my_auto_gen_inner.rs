use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::my_auto_gen::ToolFunctionAbstract;

pub struct MyAutoGenInner {
    tool_functions:
        Arc<Mutex<HashMap<&'static str, Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>>>>,
}

impl MyAutoGenInner {
    pub async fn register(
        &self,
        func_name: &'static str,
        tool_function: Arc<dyn ToolFunctionAbstract + Send + Sync + 'static>,
    ) {
        let mut write_access = self.tool_functions.lock().await;
        write_access.insert(func_name, tool_function);
    }

    pub async fn execute(&self, func_name: &str, params: &str) -> String {
        let func = {
            let read_access = self.tool_functions.lock().await;
            read_access.get(func_name).cloned()
        };

        if func.is_none() {
            panic!("tool_call with func_name '{}' is not registered", func_name);
        }

        let func = func.unwrap();

        func.call(params).await
    }
}
