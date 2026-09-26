use std::sync::Arc;

use koof::reconciler::{ControllerContext, run};

// use crate::generated::{dns_record, pages_project, zone, zone_ruleset};
use crate::generated::account;

pub async fn run_all_controllers(context: Arc<ControllerContext>) {
    let _ = tokio::join!(
        run::<account::Account>(context.clone()),
        // run::<dns_record::DNSRecord>(context.clone()),
        // run::<zone::Zone>(context.clone()),
        // run::<zone_ruleset::Ruleset>(context.clone()),
        // run::<pages_project::PagesProject>(context.clone()),
    );
}
