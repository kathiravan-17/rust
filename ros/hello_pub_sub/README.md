# ROS 2 Hello World Publisher / Subscriber (Rust)

A minimal ROS 2 talker/listener written in Rust using the [`r2r`](https://github.com/sequenceplanner/r2r) crate.

- `src/bin/talker.rs` publishes `"Hello World: N"` on `/chatter` every second.
- `src/bin/listener.rs` subscribes to `/chatter` and prints each message.

## Prerequisites (Ubuntu 22.04)

ROS 2 Humble and libclang (used by r2r to generate message bindings):

```bash
sudo apt install -y ros-humble-ros-base libclang-dev
```

Rust stable. This project pins `stable` via `rust-toolchain.toml`, so rustup picks it automatically:

```bash
rustup update stable
```

## Build

```bash
source /opt/ros/humble/setup.bash
cd ros/hello_pub_sub
cargo build
```

## Run

Terminal 1:

```bash
source /opt/ros/humble/setup.bash
cd ros/hello_pub_sub && cargo run --bin talker
```

Terminal 2:

```bash
source /opt/ros/humble/setup.bash
cd ros/hello_pub_sub && cargo run --bin listener
```

Expected output:

```
Publishing: 'Hello World: 1'      # talker
I heard: 'Hello World: 1'         # listener
```

You can also inspect the topic with the ROS CLI:

```bash
ros2 topic echo /chatter
```

Tip: add `source /opt/ros/humble/setup.bash` to `~/.bashrc` to avoid sourcing in every terminal.
