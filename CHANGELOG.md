# Changelog

In this document, all remarkable changes are listed. Not mentioned are smaller code cleanups or documentation improvements.

## Unreleased

- Added `get_current_state()` function to `GodotGGRSP2PSession` which returns the current state as a String.
- Added `get_current_state()` function to `GodotGGRSP2PSpectatorSession` which returns the current state as a String.
- Added `get_network_stats(handle)` function to `GodotGGRSP2PSession` which returns the network stats of the handle as a tuple.
- Added `get_network_stats()` function to `GodotGGRSP2PSpectatorSession` which returns the network stats as a tuple.

## 0.3.1

- Updated GGRS to v0.5.0

## 0.3.0

- Added `get_events()` function to `GodotGGRSP2PSpectatorSession` which calls `events()` on the GGRS Session
- Added `get_events()` function to `GodotGGRSP2PSession` which calls `events()` on the GGRS Session

## 0.2.0

- Implemented SyncTestSession
- Implemented P2PSpectatorSession
- Implemented additional setters for P2PSession
- Improved code architecture

## 0.1.0

- `P2PSession` has been implemented and tested in Godot 3.3.2