use std::sync::Arc;

// use crate::generated::{account, dns_record, pages_project, zone, zone_ruleset};
use crate::generated::account;

pub async fn run_all_controllers(
    context: Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) {
    let _ = tokio::join!(
        account::run_controller(context.clone()),
        // dns_record::run_controller(context.clone()),
        // zone::run_controller(context.clone()),
        // zone_ruleset::run_controller(context.clone()),
        // pages_project::run_controller(context.clone()),
    );
}
