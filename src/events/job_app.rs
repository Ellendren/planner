use time::OffsetDateTime;

#[derive(Debug)]
pub struct JobApp {
    position: String,
    company: String,
    date_applied: OffsetDateTime,
    update_date: OffsetDateTime,
    platform: String,
    status: JobStatus,
    job_description: String,
}

#[derive(Debug)]
pub enum  JobStatus {
    Applied,
    Rejected,
    InterveiwPending {
        round: u8,
        date: OffsetDateTime
    },
    OfferPending
}

impl JobApp {
    pub fn new_app (
        position: String,
        company: String,
        date_applied: OffsetDateTime,
        update_date: OffsetDateTime,
        platform: String,
        job_description: String
    ) -> Self {
        JobApp {
            position,
            company,
            date_applied,
            update_date,
            platform,
            status: JobStatus::Applied,
            job_description
        }
    }
}