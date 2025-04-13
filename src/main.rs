mod audio;
mod video;

fn main() {
	// Accept video file as input

	// Pass file to video function and audio function to be processed
	video::convert_video();
	audio::convert_audio();

	// Combine results of said functions into output video

	// Save

	todo!();
}
