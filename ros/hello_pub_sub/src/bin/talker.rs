use r2r::std_msgs::msg::String as StringMsg;
use r2r::QosProfile;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "talker", "")?;
    let publisher = node.create_publisher::<StringMsg>("/chatter", QosProfile::default())?;

    let mut count = 0u64;
    loop {
        let msg = StringMsg { data: format!("Hello World: {count}") };
        publisher.publish(&msg)?;
        println!("Publishing: '{}'", msg.data);
        count += 1;
        node.spin_once(Duration::from_millis(10));
        std::thread::sleep(Duration::from_secs(1));
    }
}
