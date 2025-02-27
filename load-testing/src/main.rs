use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use rlt::{cli::BenchCli, IterInfo, IterReport, BenchSuite};
use tokio::time::Instant;
use reqwest::{Client, Url};

/// Lets to gets all program arguments into setup configuration.
#[derive(Parser, Clone)]
struct HttpPostBench {
    // Target URL
    pub url: Url,

    // Post json
    pub json: String,

    /// Embed BenchOpts into this Opts.
    #[command(flatten)]
    pub bench_opts: BenchCli,
}

/// Send post request according to HttpPostBench configuration.
#[async_trait]
impl BenchSuite for HttpPostBench {
    type WorkerState = Client;

    async fn state(&self, _: u32) -> Result<Self::WorkerState> {
        Ok(Client::new())
    }

    async fn bench(&mut self, client: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let t = Instant::now();

        let resp = client.post(self.url.clone())
            .header("Content-Type", "application/json; charset=utf-8")
            .body(self.json.clone())
            .send().await?;


        let status = resp.status().into();
        let bytes = resp.bytes().await?.len() as u64;
        let duration = t.elapsed();
        Ok(IterReport { duration, status, bytes, items: 1 })
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let bs = HttpPostBench::parse();
    rlt::cli::run(bs.bench_opts, bs).await
}
