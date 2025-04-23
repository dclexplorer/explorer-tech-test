use godot::prelude::*;

#[cfg(feature = "use_comms")]
use comms::*;

#[cfg(feature = "use_runtime")]
use runtime::*;

// Deriving GodotClass makes the class available to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    base: Base<Node>,
}

#[godot_api]
impl Main {
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Hello from Rust 1");

        #[cfg(feature = "use_comms")]
        {
            run_livekit_demo_background();
        }
    
        #[cfg(feature = "use_runtime")]
        {
            godot_print!("RUNTIME: {}", runtime_main().to_godot());
        }
    }
}
