use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::{spawn_local, LocalSet};
use tokio::time::sleep;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create runtime");

    let local = LocalSet::new();
    let _guard = local.enter();

    let (tx, rx) = oneshot::channel();

    local.block_on(&rt, async move {
        spawn_local(async move {
            sleep(Duration::ZERO).await;

            tx.send(()).expect("failed to send");
        });
        assert_eq!(rx.await, Ok(()));
    });
}
