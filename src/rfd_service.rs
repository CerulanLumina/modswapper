use std::collections::BTreeMap;
use std::sync::RwLock;

use crossbeam::channel::internal::SelectHandle;
use crossbeam::channel::Receiver;
use once_cell::sync::Lazy;
use uuid::Uuid;

pub static RFD_INVOKER: Lazy<RFDInvoker> = Lazy::new(|| RFDInvoker {
    in_flight: RwLock::new(BTreeMap::new()),
});

pub struct RFDInvoker {
    in_flight: RwLock<BTreeMap<Uuid, RFDInvocation>>,
}

struct RFDInvocation {
    pub recv: BTreeMap<usize, Receiver<Option<String>>>,
}

impl RFDInvoker {
    pub fn open_dialog(&self, uuid: Uuid, index: usize) {
        let (sender, recv) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            let out = futures::executor::block_on(async {
                let handle = rfd::AsyncFileDialog::new().pick_folder().await;
                handle.and_then(|a| a.path().to_str().map(str::to_owned))
            });
            sender.send(out).ok(); // TODO: Log disconnection
        });

        let mut in_flight = self.in_flight.write().expect("Locking");
        let invocation = in_flight.entry(uuid).or_insert(RFDInvocation {
            recv: BTreeMap::new(),
        });
        drop(invocation.recv.insert(index, recv));
    }

    pub fn poll(&self, uuid: &Uuid) -> Option<(String, usize)> {
        let in_flight = self.in_flight.read().expect("Locking");
        let inv = in_flight.get(uuid)?;
        let ret = inv
            .recv
            .iter()
            .find(|(_, recv)| recv.is_ready())
            .map(|(idx, recv)| (*idx, recv.recv().expect("receiving")))
            .and_then(|(idx, maybe_path)| maybe_path.map(|path| (path, idx)));
        drop(in_flight);

        if let Some((_, idx)) = &ret {
            let mut in_flight = self.in_flight.write().unwrap();
            let inv = in_flight.get_mut(uuid).unwrap();
            inv.recv.remove(idx);
            if inv.recv.is_empty() {
                in_flight.remove(uuid);
            }
        }

        ret
    }
}
