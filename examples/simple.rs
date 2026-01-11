use bevy::app::{App, Update};
use bevy::prelude::{Local, MessageWriter, Timer, TimerMode};
use bevy::MinimalPlugins;
use bevy_trackpad_haptic::{FeedbackMessage, TrackpadHapticPlugin};
use std::time::{Duration, Instant};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, TrackpadHapticPlugin))
        .add_systems(Update, trigger)
        .run();
}
fn trigger(
    mut timer: Local<Timer>,
    mut last_trigger_instant: Local<Option<Instant>>,
    mut feedback_event_writer: MessageWriter<FeedbackMessage>,
) {
    const DURATION: Duration = Duration::from_secs(1);

    // Using real-time instead of bevy's due to MinimalPlugins.
    let last_trigger_time = last_trigger_instant.get_or_insert_with(Instant::now);
    let now = Instant::now();
    timer.tick(now.duration_since(*last_trigger_time));
    *last_trigger_instant = Some(now);

    if timer.is_finished() {
        println!("Triggered feedback");
        feedback_event_writer.write(FeedbackMessage::new(100, 0));
        *timer = Timer::new(DURATION, TimerMode::Repeating);
    }
}
