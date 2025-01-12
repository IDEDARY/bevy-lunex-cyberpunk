use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use vleue_kinetoscope::*;

#[derive(Component)]
pub struct Movie {
    pub video_src: Handle<AnimatedImage>,
    pub audio_src: Handle<AudioSource>,
    audio_ins: Option<Handle<AudioInstance>>,
}
impl Movie {
    pub fn play(video_src: Handle<AnimatedImage>, audio_src: Handle<AudioSource>) -> Self {
        Self {
            video_src,
            audio_src,
            audio_ins: None,
        }
    }
    // System for controlling the movie playback
    fn system(mut commands: Commands, mut movies: Query<(Option<&mut AnimatedImageController>, &mut Movie, Entity)>, mut audio_assets: ResMut<Assets<AudioInstance>>, audio: Res<Audio>) {
        for (controller_option, mut movie, entity) in &mut movies {

            // Start playing if it is not already
            if controller_option.is_none() && movie.audio_ins.is_none() {
                commands.entity(entity).insert(AnimatedImageController::play(movie.video_src.clone()));
                movie.audio_ins = Some(audio.play(movie.audio_src.clone()).handle());
            
            //
            } else if let (Some(audio_instance), Some(mut controller)) = (audio_assets.get_mut(movie.audio_ins.as_ref().expect("Must be set")), controller_option) {

                // Check if both are stopped
                let video_stopped = controller.current_frame() == controller.frame_count() || controller.play_count() >= 1;
                let audio_stopped = audio_instance.state() == PlaybackState::Stopped;

                // Stop movie from looping if it ended
                if video_stopped && !controller.paused() { controller.pause(); warn!("Video track ended!") }

                // Reset the movie once both tracks stopped
                if video_stopped && audio_stopped {
                    warn!("Video track and Sound track ended, Restarting movie...");
                    controller.reset();
                    movie.audio_ins = Some(audio.play(movie.audio_src.clone()).handle());
                }
            }
        }
    }
}

/// Plugin with VFX systems for our menu
pub struct MoviePlugin;
impl Plugin for MoviePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Movie::system);
    }
}