use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};
use std::{
    cell::{RefCell, RefMut},
    io::Cursor,
    rc::Rc,
    time::{Duration, Instant},
};

const TICK: &[u8] = include_bytes!("../assets/audio/tick.flac");
const BELL_A: &[u8] = include_bytes!("../assets/audio/bell-a.flac");
const BELL_B: &[u8] = include_bytes!("../assets/audio/bell-b.flac");
const TICK_CURSOR: Cursor<&[u8]> = Cursor::new(TICK);
const BELL_A_CURSOR: Cursor<&[u8]> = Cursor::new(BELL_A);
const BELL_B_CURSOR: Cursor<&[u8]> = Cursor::new(BELL_B);
const AUDIO_IDLE_GRACE: Duration = Duration::from_secs(30);

struct AudioInner {
    sink: Option<MixerDeviceSink>,
    player: Option<Player>,
    last_use: Option<Instant>,
}

impl AudioInner {
    fn new() -> Self {
        Self {
            sink: None,
            player: None,
            last_use: None,
        }
    }

    fn player(&mut self) -> Option<&Player> {
        if self.player.is_none() {
            match DeviceSinkBuilder::open_default_sink() {
                Ok(mut sink) => {
                    sink.log_on_drop(false);
                    let player = Player::connect_new(&sink.mixer());
                    player.set_volume(0.5);
                    self.sink = Some(sink);
                    self.player = Some(player);
                }
                Err(e) => {
                    eprintln!("Could not open audio device: {e}");
                    return None;
                }
            }
        }
        self.last_use = Some(Instant::now());
        self.player.as_ref()
    }
    fn play(&mut self, cursor: Cursor<&'static [u8]>) {
        if let Some(player) = self.player() {
            player.append(Decoder::new(cursor).unwrap());
        }
    }

    /// Drop the device once nothing has played for `grace`.
    /// Reopening costs ~13ms, so closing between ticks would burn ~1.2% CPU.
    fn close_if_idle(&mut self) -> bool {
        let idle = self.player.as_ref().is_some_and(Player::empty)
            && self
                .last_use
                .is_some_and(|t| t.elapsed() >= AUDIO_IDLE_GRACE);
        if !idle {
            return false;
        }
        self.player = None;
        self.sink = None; // drops cpal::Stream, which closes the ALSA/PipeWire device
        self.last_use = None;
        true
    }
}

#[derive(Clone)] // Cloning just increments the Rc reference count
pub struct Audio(Rc<RefCell<AudioInner>>);

impl Audio {
    // 3. Expose a constructor that creates the wrapped type immediately
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(AudioInner::new())))
    }

    fn borrow_mut(&self) -> RefMut<'_, AudioInner> {
        self.0.borrow_mut()
    }

    pub fn play_tick(&self) {
        self.borrow_mut().play(TICK_CURSOR);
    }
    pub fn play_focus_bell(&self) {
        self.borrow_mut().play(BELL_A_CURSOR);
    }
    pub fn play_break_bell(&self) {
        self.borrow_mut().play(BELL_B_CURSOR);
    }
    pub fn close_if_idle(&mut self) -> bool {
        self.borrow_mut().close_if_idle()
    }
}
