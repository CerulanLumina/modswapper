use camino::Utf8PathBuf;
use crossbeam::channel::{Receiver, Sender, TryRecvError};

#[derive(Clone)]
pub struct RFDInvoker {
    sender: Sender<()>,
    receiver: Receiver<Option<String>>,
}

impl RFDInvoker {
    pub fn new() -> Self {
        let (sender, receiver) = spawn_rfd_thread();
        RFDInvoker { sender, receiver }
    }

    pub fn open_file_dialog(&self) {
        self.sender.send(()).ok();
    }

    pub fn latest_file_picked(&self) -> Option<Option<String>> {
        match self.receiver.try_recv() {
            Ok(item) => Some(item),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected!"),
        }
    }
}

fn spawn_rfd_thread() -> (Sender<()>, Receiver<Option<String>>) {
    let ((invoker, invoker_recv), (responder, responder_recv)) = (
        crossbeam::channel::unbounded(),
        crossbeam::channel::unbounded(),
    );
    std::thread::spawn(move || {
        for _ in invoker_recv {
            let res = rfd::FileDialog::new()
                .pick_folder()
                .and_then(|a| Utf8PathBuf::from_path_buf(a).ok())
                .map(Utf8PathBuf::into_string);
            responder.send(res).ok();
        }
    });

    (invoker, responder_recv)
}
