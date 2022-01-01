pub const NAME: &str = "check_hpe_oneview_alerts";
pub const VERSION: &str = "1.0.0";
pub const HPE_ONEVIEW_API_VERSION: &str = "1000";

pub fn generate_user_agent() -> String {
    format!("{}/{}", NAME, VERSION)
}
