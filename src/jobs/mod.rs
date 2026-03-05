use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn create_periodic_movie_fetch_job() -> Result<JobScheduler> {
    let job = JobScheduler::new().await?;

    job.add(Job::new_async("0/5 * * * * *", |uuid, j| {
        Box::pin(async { println!("Next will be in 5 second") })
    })?)
    .await?;

    Ok(job)
}
