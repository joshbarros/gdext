/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use godot_cell::blocking::GdCell;

/// A mutable borrow must wait for a shared borrow still held by another thread, even when the
/// calling thread took the first shared borrow earlier.
#[test]
#[cfg_attr(
    all(target_family = "wasm", not(target_feature = "atomics")),
    ignore = "Threading not available"
)]
fn mutable_borrow_waits_for_other_threads_shared_borrow() {
    let cell = GdCell::new(0);
    let cell = &cell;
    let (hold, held) = mpsc::channel();
    let (bound, taken) = mpsc::channel();

    thread::scope(|scope| {
        let first = cell.borrow().unwrap();
        scope.spawn(move || {
            held.recv().unwrap();
            let other = cell.borrow().unwrap();
            bound.send(()).unwrap();
            thread::sleep(Duration::from_millis(50));
            drop(other);
        });
        hold.send(()).unwrap();
        taken.recv().unwrap();
        drop(first);

        assert!(cell.borrow_mut().is_ok());
    });
}
