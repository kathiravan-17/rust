use futures::{executor::LocalPool, future, task::LocalSpawnExt, StreamExt};
use r2r::std_msgs::msg::String as StringMsg;
use r2r::QosProfile;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "listener", "")?;
    let sub = node.subscribe::<StringMsg>("/chatter", QosProfile::default())?;

    let mut pool = LocalPool::new();
    pool.spawner().spawn_local(async move {
        sub.for_each(|msg| {
            println!("I heard: '{}'", msg.data);
            future::ready(())
        })
        .await
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
