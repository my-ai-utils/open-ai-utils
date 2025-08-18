pub const SYSTEM_ROLE: &'static str = "system";
pub const USER_ROLE: &'static str = "user";

pub const ASSISTANT_ROLE: &'static str = "assistant";

pub const TOOL_ROLE: &'static str = "tool";

pub trait MessageRole {
    fn get_role(&self) -> &str;

    fn is_system(&self) -> bool {
        self.get_role() == crate::request_builder::roles::SYSTEM_ROLE
    }

    fn is_user(&self) -> bool {
        self.get_role() == crate::request_builder::roles::USER_ROLE
    }

    fn is_assistant(&self) -> bool {
        self.get_role() == crate::request_builder::roles::ASSISTANT_ROLE
    }

    fn is_tool(&self) -> bool {
        self.get_role() == crate::request_builder::roles::TOOL_ROLE
    }
}
