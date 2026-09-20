// Define a camera structure.
struct Camera {
    id: u32,
    frame_count: u32,
}

// Implement functions for Camera.
impl Camera {

    // Constructor: create a new camera.
    fn new(id: u32) -> Self {
        Self {
            id,
            frame_count: 0,
        }
    }

    // &mut self allows modification of frame_count.
    fn capture(&mut self) {

        // Increment the frame counter.
        self.frame_count += 1;

        println!(
            "Camera {} captured frame {}",
            self.id,
            self.frame_count
        );
    }

    // &self only reads data.
    fn status(&self) {
        println!(
            "Camera {} total frames: {}",
            self.id,
            self.frame_count
        );
    }
}

fn main() {

    // Create a mutable camera object.
    let mut camera = Camera::new(1);

    // Capture five frames.
    for _ in 0..5 {
        camera.capture();
    }

    // Print camera status.
    camera.status();
}