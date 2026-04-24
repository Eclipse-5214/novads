# NovaDS

A cross-platform FRC Driver Station built with Tauri 2, SvelteKit, and Rust. Runs on macOS, Windows, Linux, and Android.

## Features

- **FRC 2026 protocol** — full UDP + TCP channel (extends FRC 2020)
- **Robot control** — enable/disable, e-stop, Teleop/Auto/Test modes
- **Alliance & station** — Red/Blue 1-3 selection
- **Direct IP mode** — derives `10.TE.AM.2` from team number to bypass mDNS (required on Android)
- **Simulation mode** — connects to a WPILib simulation at `localhost`
- **Real-time status** — communications, robot code, battery voltage
- **NetConsole** — robot stdout and error output via TCP
- **Joystick support** — gamepad input forwarded to the robot
- **Persistent settings** — team number, alliance, and mode saved across sessions

## Stack

| Layer | Technology |
|---|---|
| UI | SvelteKit 2 + Tailwind CSS 4 |
| Desktop shell | Tauri 2 (Rust) |
| Protocol | LibDS (C) — FRC communication library |
| Bindings | bindgen — auto-generated Rust FFI from LibDS headers |
| Mobile | Tauri Android target |

## Prerequisites

- [Rust](https://rustup.rs/) + `cargo`
- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/)
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform
- For Android: Android SDK + NDK, `ANDROID_HOME` set

## Development

```bash
pnpm install
pnpm tauri dev
```

### Android

```bash
pnpm tauri android dev
```

## Build

```bash
pnpm tauri build
```

```bash
pnpm tauri android build
```

## Project Structure

```
src/                        SvelteKit frontend
  routes/+page.svelte       Main UI
  libs/
    api.ts                  Tauri command wrappers & settings helpers
    components/             UI components (ControlPanel, Settings, Logger, …)

src-tauri/
  src/
    lib.rs                  Tauri command registration
    commands/               robot.rs — DS control commands
  libds/                    LibDS C library (vendored, FRC protocol engine)
    src/protocols/
      frc_2026.c            FRC 2026 protocol (UDP + TCP)
      frc_2020.c            FRC 2020 protocol (base)
  build.rs                  Compiles LibDS and generates FFI bindings
```

## Protocol Notes

NovaDS uses the **FRC 2026** protocol by default, which extends FRC 2020 with a TCP channel on port 1740 for:
- Joystick descriptors
- Robot stdout / stderr (NetConsole)
- Error and fault reporting

On Android, the app acquires a `WifiManager.MulticastLock` to allow inbound UDP from the robot. Direct IP mode is recommended on Android since mDNS (`.local` hostnames) is not available via bionic's `getaddrinfo`.

## Credits

Protocol engine: [LibDS](https://github.com/FRC-Utilities/LibDS) by Alex Spataru
