use hydro_deploy::Deployment;

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();

    let flow = hydro_lang::FlowBuilder::new();
    let process = flow.process();
    hydro_template::hello_world_hydro::hello_world_hydro(&process);

    let _nodes = flow
        .with_process(&process, deployment.Localhost())
        .deploy(&mut deployment);

    deployment.run_ctrl_c().await.unwrap();
}
