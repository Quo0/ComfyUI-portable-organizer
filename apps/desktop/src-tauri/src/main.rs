// Copyright (C) 2026 Andrew Blokhin
// SPDX-License-Identifier: GPL-3.0-only

// Do not remove: without this a console opens next to the window in a release build.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    cpo_desktop_lib::run()
}
