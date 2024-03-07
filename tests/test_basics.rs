#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    //let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract_wasm = include_bytes!("../pycontract.wasm");

    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let user_message_outcome = contract.view("method").await?;

    Ok(())
}
