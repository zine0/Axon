use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Judger service started");

    loop {
        match check_for_submissions().await {
            Ok(_) => {}
            Err(e) => tracing::error!("Error checking submissions: {}", e),
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_for_submissions() -> anyhow::Result<()> {
    tracing::debug!("Checking for new submissions...");
    Ok(())
}
