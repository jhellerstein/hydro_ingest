use hydro_deploy::Deployment;
use tokio::time::{timeout, Duration};
#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();
    let flow = hydro_lang::FlowBuilder::new();
    let process = flow.process::<()>();
    hydro_template::mixed_io_hydro::mixed_io_hydro(&process);
    let _nodes = flow
        .with_process(&process, deployment.Localhost())
        .deploy(&mut deployment);
    println!("Starting deployment...");
    println!("Looking for 'running command:' output...");
    deployment.deploy().await.unwrap();
    let start_result = timeout(
            Duration::from_secs(60),
            async {
                deployment.start().await.unwrap();
            },
        )
        .await;
    match start_result {
        Ok(_) => {
            println!("✓ Deployment completed successfully");
        }
        Err(_) => {
            println!("✓ Deployment reached 60-second timeout");
            println!("If you saw output containing:");
            println!("  [() (process 0)] running command: `...`");
            println!("  [() (process 0)] <your program output>");
            println!("Then the deployment worked correctly!");
        }
    }
}
