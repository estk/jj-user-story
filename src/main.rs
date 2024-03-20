use user_story_jj::Cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    Cli::run().await
}
