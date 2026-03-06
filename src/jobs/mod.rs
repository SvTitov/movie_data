use std::{os::macos::raw::stat, sync::Arc};

use anyhow::Result;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::AppState;

pub async fn create_periodic_movie_fetch_job(state: Arc<Mutex<AppState>>) -> Result<JobScheduler> {
    let job = JobScheduler::new().await?;

    let connector = state.lock().await.omdb_connector.clone();

    job.add(Job::new_async("0/5 * * * * *", move |uuid, j| {
        let connector = connector.clone();
        Box::pin(async move {
            let guard = connector.get_info("").await;

            println!("Next will be in 5 second")
        })
    })?)
    .await?;

    Ok(job)
}
