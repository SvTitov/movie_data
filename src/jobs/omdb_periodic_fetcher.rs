use crate::{connectors::omdb::OmdbConnector, dal::persistent_repo::PersistentRepo};
use anyhow::Result;
use std::{cell::RefCell, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(Default)]
enum State {
    #[default]
    NotStarted,
    Running,
    Stopped,
}

pub struct OmdbPeriodicFetcher {
    state: RefCell<State>,
    storage: Arc<dyn PersistentRepo>,
}

impl OmdbPeriodicFetcher {
    pub fn new(storage: Arc<dyn PersistentRepo>) -> Self {
        Self {
            storage,
            state: RefCell::new(Default::default()),
        }
    }

    pub async fn start_fetch(&self, connector: Arc<OmdbConnector>) -> Result<JobScheduler> {
        self.change_state(State::Running);

        let job = JobScheduler::new().await?;

        job.add(Job::new_async("0/5 * * * * *", move |_uuid, _j| {
            let connector = connector.clone();

            Box::pin(async move {
                println!("Before request terminator");

                let info = connector.get_info("terminator").await;

                if info.is_err() {
                    println!("This is an error")
                }

                match info {
                    Ok(_info) => println!("{:?}", _info),
                    Err(err) => println!("{}", err),
                }

                println!("Next will be in 5 second")
            })
        })?)
        .await?;

        Ok(job)
    }

    fn change_state(&self, state: State) {
        let mut s = self.state.borrow_mut();
        *s = state;
    }
}
