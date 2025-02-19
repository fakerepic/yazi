use std::{collections::VecDeque, ffi::OsString};

use crossterm::event::KeyEvent;
use tokio::sync::{mpsc, oneshot};

use super::Cmd;
use crate::{term::Term, Layer, RoCell};

static TX: RoCell<mpsc::UnboundedSender<Event>> = RoCell::new();

#[derive(Debug)]
pub enum Event {
	Call(Cmd, Layer),
	Seq(VecDeque<Cmd>, Layer),
	Render,
	Key(KeyEvent),
	Resize,
	Paste(String),
	Quit(EventQuit),
}

#[derive(Debug, Default)]
pub struct EventQuit {
	pub no_cwd_file: bool,
	pub selected:    Option<OsString>,
}

impl Event {
	#[inline]
	pub fn init(tx: mpsc::UnboundedSender<Event>) { TX.init(tx); }

	#[inline]
	pub fn emit(self) { TX.send(self).ok(); }

	#[inline]
	pub async fn wait<T>(self, rx: oneshot::Receiver<T>) -> T {
		TX.send(self).ok();
		rx.await.unwrap_or_else(|_| Term::goodbye(|| false))
	}
}

#[macro_export]
macro_rules! emit {
	(Quit($opt:expr)) => {
		$crate::event::Event::Quit($opt).emit();
	};
	(Call($cmd:expr, $layer:expr)) => {
		$crate::event::Event::Call($cmd, $layer).emit();
	};
	(Seq($cmds:expr, $layer:expr)) => {
		$crate::event::Event::Seq($cmds, $layer).emit();
	};
	($event:ident) => {
		$crate::event::Event::$event.emit();
	};
}
