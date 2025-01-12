use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use vleue_kinetoscope::*;

#[derive(Component)]
pub struct Movie {
    pub playback: MoviePlayback,
    pub video_src: Handle<AnimatedImage>,
    pub audio_src: Handle<AudioSource>,
    audio_ins: Option<Handle<AudioInstance>>,
    state: MovieState,

}
impl Movie {
    /// Play new movie
    pub fn play(video_src: Handle<AnimatedImage>, audio_src: Handle<AudioSource>) -> Self {
        Self {
            playback: MoviePlayback::Despawn,
            video_src,
            audio_src,
            audio_ins: None,
            state: MovieState::Playing,
        }
    }
    /// Set the movie playback
    pub fn playback(mut self, playback: MoviePlayback) -> Self {
        self.playback = playback;
        self
    }
    /// System for controlling the movie playback
    fn system(mut commands: Commands, mut movies: Query<(Option<&mut AnimatedImageController>, &mut Movie, Entity)>, mut audio_assets: ResMut<Assets<AudioInstance>>, audio: Res<Audio>, mut event: EventWriter<MovieEnded>) {
        for (controller_option, mut movie, entity) in &mut movies {

            // Start playing if it is not already
            if controller_option.is_none() && movie.audio_ins.is_none() {
                commands.entity(entity).insert(AnimatedImageController::play(movie.video_src.clone()));
                movie.audio_ins = Some(audio.play(movie.audio_src.clone()).handle());
            
            } else if let (Some(audio_instance), Some(mut controller)) = (audio_assets.get_mut(movie.audio_ins.as_ref().expect("Must be set")), controller_option) {

                // Check if both are stopped
                let video_stopped = controller.current_frame() == controller.frame_count() || controller.play_count() >= 1;
                let audio_stopped = audio_instance.state() == PlaybackState::Stopped;

                // Stop movie from looping if it ended
                if video_stopped && !controller.paused() { controller.pause();}

                // Movie ended
                if video_stopped && audio_stopped && movie.state != MovieState::Ended {
                    match movie.playback {
                        MoviePlayback::Repeat => {
                            controller.reset();
                            movie.audio_ins = Some(audio.play(movie.audio_src.clone()).handle());
                        }
                        MoviePlayback::Despawn => {
                            commands.entity(entity).despawn_recursive();
                        }
                        _ => {
                            movie.state = MovieState::Ended
                        },
                    }
                    event.send(MovieEnded { target: entity });
                }
            }
        }
    }
}

/// What should the move do after it ends
pub enum MoviePlayback {
    /// Do nothing
    Stop,
    /// Repeat itself
    Repeat,
    /// Despawn itself
    Despawn,
}

/// The current state of the movie
#[derive(PartialEq)]
pub enum MovieState {
    Playing,
    Ended,
}

#[derive(Event)]
pub struct MovieEnded {
    pub target: Entity
}

/// Plugin with VFX systems for our menu
pub struct MoviePlugin;
impl Plugin for MoviePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovieEnded>();
        app.add_systems(Update, Movie::system);
    }
}