//! Event handling module for the Zack text editor.
//!
//! This module provides an `EventHandler` that handles asynchronous input events
//! from the terminal (via Crossterm) and application-specific events. It emits events
//! at a fixed frame rate (`TICK_FPS`) and uses a separate thread to poll for input,
//! enabling responsive and concurrent input handling.
//!
//! Events handled include:
//! - `Crossterm` input events (keyboard, mouse, resize, etc.)
//! - Application-specific events (`AppEvent`)
//! - Periodic `Tick` events at 30 FPS

use super::app_events::AppEvent;
use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{self, Event as CrosstermEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const TICK_FPS: f64 = 30.0;

/// Enum representing all types of events handled by the editor.
#[derive(Clone, Debug)]
pub enum Event {
    /// Emitted periodically to trigger UI updates or animations.
    Tick,

    /// A raw input event received from Crossterm (keyboard, mouse, etc.).
    Crossterm(CrosstermEvent),

    /// A high-level application-specific event.
    App(AppEvent),
}

/// Central struct responsible for managing and dispatching events to the application.
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    /// Creates a new `EventHandler` and spawns a background thread to emit events.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let actor = EventThread::new(sender.clone());
        thread::spawn(|| actor.run());
        Self { sender, receiver }
    }

    /// Receives the next event from the internal queue (blocking).
    pub fn next(&self) -> color_eyre::Result<Event> {
        Ok(self.receiver.recv()?)
    }

    /// Sends a custom `AppEvent` into the event stream.
    pub fn send(&mut self, app_event: AppEvent) {
        let _ = self.sender.send(Event::App(app_event));
    }
}

/// Background actor responsible for polling terminal events and sending periodic ticks.
struct EventThread {
    sender: mpsc::Sender<Event>,
}

impl EventThread {
    /// Creates a new `EventThread` with the given sender.
    fn new(sender: mpsc::Sender<Event>) -> Self {
        Self { sender }
    }

    /// Runs the event loop, emitting `Tick` events and handling Crossterm input.
    fn run(self) -> color_eyre::Result<()> {
        let tick_interval = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_interval.saturating_sub(last_tick.elapsed());
            if timeout == Duration::ZERO {
                last_tick = Instant::now();
                self.send(Event::Tick);
            }
            if event::poll(timeout).wrap_err("failed to poll for crossterm events")? {
                let event = event::read().wrap_err("failed to read crossterm event")?;
                self.send(Event::Crossterm(event));
            }
        }
    }

    /// Sends an event through the channel.
    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
