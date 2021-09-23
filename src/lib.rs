/*
*   This crate is a scrap. It defines code and data modularly.
*
*/

use engine_macros::glue_runloop;

use engine_use::{
	run::run_loop::{RunLoop, RunLoopId},
	sync,
};

use serde::Deserialize;

// Here we define a struct that contains data underlying the RunLoop. Keep in mind that this data cannot be accessed by other RunLoops
#[derive(Debug, Deserialize)]
pub struct VoxelRendererRunLoop {
	#[serde(default = "RunLoopId::default")]
	run_loop_id: RunLoopId,
	some_val_1: u32,
	some_val_2: bool,
}

glue_runloop!(VoxelRendererRunLoop);

impl RunLoop for VoxelRendererRunLoop {
	fn run_loop_id(&self) -> RunLoopId {
		self.run_loop_id.clone()
	}

	fn run(&self) {
		println!("RunLoop entered");
		//one-time setup
		loop {
			sync::wait_for_parents(self.run_loop_id());
			//do stuff
			sync::release_children(self.run_loop_id());
		}
	}
}
