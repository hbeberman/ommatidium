# Ommatidium

A small, dependency-free Rust library for managing hierarchichal TUIs.

## Session
This is your main handle into Ommatidium, you can only initialize one of these.

## Window
This is the bread and butter.

* Each window gets a monotonically increasing unique ID upon being added to the session
* Windows have a list of Windows as children
