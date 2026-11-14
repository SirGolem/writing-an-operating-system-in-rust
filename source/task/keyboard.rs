use crate::{print, print_line};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{Stream, StreamExt, task::AtomicWaker};
use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1, layouts::Us104Key};

static SCAN_CODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn push_scan_code(scan_code: u8) -> () {
    if let Ok(scan_code_queue) = SCAN_CODE_QUEUE.try_get() {
        if let Err(_) = scan_code_queue.push(scan_code) {
            print_line!("Warning: Scan code queue full; keyboard input dropped.")
        } else {
            WAKER.wake();
        }
    } else {
        print_line!("Warning: Scan code queue uninitialized.")
    }
}

pub async fn print_key_presses() -> () {
    let mut keyboard = PS2Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore);
    let mut scan_code_stream = ScanCodeStream::new();

    while let Some(scan_code) = scan_code_stream.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    | DecodedKey::RawKey(key) => print!("{key:?}"),
                    | DecodedKey::Unicode(character) => print!("{character}"),
                }
            }
        }
    }
}

struct ScanCodeStream {
    _private: (),
}

impl ScanCodeStream {
    pub fn new() -> Self {
        SCAN_CODE_QUEUE.try_init_once(|| ArrayQueue::new(128)).expect("Scan code stream may only be initialized once");

        Self {
            _private: (),
        }
    }
}

impl Stream for ScanCodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let scan_code_queue = SCAN_CODE_QUEUE.try_get().expect("Scan code queue uninitialized");

        if let Some(scan_code) = scan_code_queue.pop() {
            return Poll::Ready(Some(scan_code));
        }

        WAKER.register(&context.waker());
        match scan_code_queue.pop() {
            | None => Poll::Pending,
            | Some(scan_code) => {
                WAKER.take();
                Poll::Ready(Some(scan_code))
            }
        }
    }
}
