use tracing::{info, warn};
use rand::Rng;
use tokio::time::{sleep, Duration};

pub struct ConnectionFramework {
    name: String,
    port: u16,
    max_tries: u32,
}

pub struct FrameworkConnection {
    pub stable: bool,
}

impl ConnectionFramework {
    pub fn new(name: String, port: u16, max_tries: u32) -> Self {
        Self { name, port, max_tries }
    }

    pub async fn stabilize_connection(&self) -> Result<FrameworkConnection, String> {
        let mut tries = 0;
        let mut rng = rand::thread_rng();

        while tries < self.max_tries {
            tries += 1;
            info!(
                "framework_connect attempt {} name={} port={}",
                tries, self.name, self.port
            );

            let ok: bool = rng.gen_bool(0.7);
            if ok {
                info!("connection stabilized on attempt {}", tries);
                return Ok(FrameworkConnection { stable: true });
            }

            warn!("connection jitter, retrying...");
            sleep(Duration::from_millis(150)).await;
        }

        warn!("max_tries reached without full stabilization");
        Ok(FrameworkConnection { stable: false })
    }
}
