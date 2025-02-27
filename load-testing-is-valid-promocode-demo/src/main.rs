use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use rlt::{cli::BenchCli, IterInfo, IterReport, BenchSuite};
use tokio::time::Instant;
use reqwest::{Client};


/// Lets to gets all program arguments into setup configuration.
#[derive(Parser, Clone)]
struct HttpPostBench {
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

        let json = "
{
  \"promocode_name\": \"WeatherCode1\",
  \"arguments\": {
    \"age\": 40,
    \"town\": \"Lyon\"
  }
}";

       let resp = client.post("http://localhost:8080/is-valid-promocode")
            .header("Content-Type", "application/json; charset=utf-8")
            .body(json)
            .send().await?;


        let status = resp.status().into();
        let bytes = resp.bytes().await?.len() as u64;
        let duration = t.elapsed();
        Ok(IterReport { duration, status, bytes, items: 1 })
    }
}

async fn add_first_promocode_to_test() {
    let json =  format!("{{
  \"name\": \"WeatherCode{:?}\",
    \"advantage\": {{ \"percent\": 20 }},
    \"restrictions\": [
      {{
        \"date\": {{
          \"after\": \"2019-01-01\",
          \"before\": \"2029-06-30\"
        }}
      }},
      {{
        \"or\": [
          {{
            \"age\": {{
              \"eq\": 40
            }}
          }},
          {{
            \"and\": [
              {{
               \"age\": {{
                  \"lt\": 30,
                  \"gt\": 15
                }}
              }},
              {{
                \"weather\": {{
\"is\": \"Clear\",
                  \"temp\": {{
                    \"gt\": 15
                  }}
                }}
              }}
            ]
          }}
        ]
      }}
    ]
}}
", 1);


        let _ = Client::new().post("http://localhost:8080/add-promocode")
            .header("Content-Type", "application/json; charset=utf-8")
            .body(json)
            .send().await;
}


#[tokio::main]
async fn main() -> Result<()> {
    add_first_promocode_to_test().await;
    let bs = HttpPostBench::parse();
    rlt::cli::run(bs.bench_opts, bs).await
}

