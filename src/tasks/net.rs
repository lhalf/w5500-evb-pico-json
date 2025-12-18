#[embassy_executor::task]
pub async fn net_task(
    mut runner: embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
) -> ! {
    runner.run().await
}
