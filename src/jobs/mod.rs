use std::sync::Arc;

use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::connectors::omdb::OmdbConnector;

pub async fn create_periodic_movie_fetch_job(
    connector: Arc<OmdbConnector>,
) -> Result<JobScheduler> {
    let job = JobScheduler::new().await?;

    job.add(Job::new_async("0/5 * * * * *", move |_uuid, _j| {
        let connector = connector.clone();

        Box::pin(async move {
            let _info = connector.get_info("").await;

            if let Ok(_info) = _info {}

            println!("Next will be in 5 second")
        })
    })?)
    .await?;

    Ok(job)
}
