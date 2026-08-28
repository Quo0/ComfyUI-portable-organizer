# EP-RUN — Launching and the lifecycle

A replacement for double-clicking a `.bat`. The app starts the ComfyUI server,
shows the logs, waits for readiness and stops it properly — including the cases
where something went wrong.

The key difference from a manual launch: no browser opens, no console window
appears, and after the app is closed no processes are left occupying the VRAM.

## Functional requirements

| ID | Requirement | Rationale in `PLAN.md` |
|---|---|---|
| `FR-RUN-010` | An instance launches with the chosen profile; the last profile used is remembered | «Парсинг .bat», «Экраны» |
| `FR-RUN-020` | When launched from the app, no browser opens | «Ключевые находки», факт 1 |
| `FR-RUN-030` | The app hands the instance a free port, respecting the preferred one | «ports.rs» |
| `FR-RUN-040` | The startup logs are visible in real time | «supervise/ + process.rs» |
| `FR-RUN-050` | The app determines the moment the server becomes ready | «supervise/ + process.rs» |
| `FR-RUN-060` | An instance's state is reflected in the interface and distinguishes ordinary outcomes from crashes | «supervise/ + process.rs» |
| `FR-RUN-070` | Stopping terminates the server and releases the port | «supervise/ + process.rs» |
| `FR-RUN-080` | Quitting the app leaves no servers running | «supervise/ + process.rs» |
| `FR-RUN-090` | The user is warned about the risks of running several instances at once | «Грабли» |
| `FR-RUN-100` | A server restart performed by ComfyUI-Manager is recognised rather than looking like a crash | «Грабли» |
| `FR-RUN-110` | The launch arguments can be viewed and changed | «Парсинг .bat» |
| `FR-RUN-120` | The app does not appear as a console window on top of the interface | «supervise/windows.rs» |

---

### US-RUN-01 — Choosing the launch profile

**As** the owner of an instance
**I want** to choose the way it is launched
**so that** I use the graphics card mode I need.

Tags: `@FR-RUN-010` `@FR-RUN-110` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «Парсинг .bat»

**Preconditions**
- The instance is registered and stopped.

**Acceptance criteria**
- **AC-1.** The available profiles are listed with understandable names.
- **AC-2.** A launch in one action uses the last profile applied.
- **AC-3.** On the first launch the build's main profile is offered by default.
- **AC-4.** The chosen profile is remembered for next time.
- **AC-5.** The user can view a profile's arguments before launching.
- **AC-6.** The user can change the arguments and save a variant of the profile as their own.

**Negative and edge cases**
- **AC-7.** If the profiles could not be recognised, the user can set the launch command by hand.
- **AC-8.** Changed arguments do not affect the build's own files.

---

### US-RUN-02 — Launching an instance

**As** a user
**I want** to launch ComfyUI from the app
**so that** I do not have to hunt for the `.bat` in Explorer or catch the tab
in the browser.

Tags: `@FR-RUN-020` `@FR-RUN-030` `@FR-RUN-120` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «Ключевые находки»

**Preconditions**
- The instance is registered, the profile is chosen.

**Acceptance criteria**
- **AC-1.** The launch begins with a single action by the user.
- **AC-2.** The default browser does not open at any moment of the launch.
- **AC-3.** No console window appears on top of the interface.
- **AC-4.** The instance receives a port: the preferred one, if it is free.
- **AC-5.** The port in use is shown to the user.
- **AC-6.** The instance's state changes to "starting" as soon as the launch begins.

**Negative and edge cases**
- **AC-7.** If the preferred port is taken, the next free one is handed out, and the user is told which one and why.
- **AC-8.** If no free port could be found, the launch does not begin, with the reason explained.
- **AC-9.** If the instance's folder is unavailable, the launch does not begin, with the reason explained.
- **AC-10.** A path with spaces and non-Latin characters does not get in the way of the launch.

---

### US-RUN-03 — Watching the startup

**As** a user who launched an instance
**I want** to see what is happening
**so that** I can tell whether it is loading or something has broken.

Tags: `@FR-RUN-040` `@FR-RUN-050` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «supervise/ + process.rs»

**Preconditions**
- The instance is in the "starting" state.

**Acceptance criteria**
- **AC-1.** The server's output is shown as it arrives, not after the startup has finished.
- **AC-2.** The first lines appear within a few seconds of the launch beginning.
- **AC-3.** Model-loading indicators that redraw a single line do not turn into thousands of lines in the log.
- **AC-4.** The user can cancel the launch while it has not finished.
- **AC-5.** On reaching readiness the state changes to "running".
- **AC-6.** A cold start lasting several minutes counts as ordinary and is not broken off early.
- **AC-7.** The log stays available after the instance has started working.

**Negative and edge cases**
- **AC-8.** If the server did not reach readiness within the time allowed, the startup is halted and the log is kept for diagnosis.
- **AC-9.** If the process finished before readiness, the state changes to crashed, and the last lines of the log are shown immediately.
- **AC-10.** Cancelling the launch leaves no running process and releases the port.

---

### US-RUN-04 — Stopping an instance

**As** a user
**I want** to stop the server
**so that** the VRAM and the port are freed.

Tags: `@FR-RUN-070` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «supervise/ + process.rs»

**Preconditions**
- The instance is running.

**Acceptance criteria**
- **AC-1.** The stop begins with a single action.
- **AC-2.** The state changes to "stopping", then to "stopped".
- **AC-3.** After the stop the server's process is not left in the system.
- **AC-4.** After the stop the port is free and can be handed to another instance.
- **AC-5.** The child processes spawned by the server are terminated too.

**Negative and edge cases**
- **AC-6.** If the process does not finish within a reasonable time, the user learns about it and can repeat the stop.
- **AC-7.** A restart is performed as a stop followed by a launch with the same profile.

---

### US-RUN-05 — Several instances running at once

**As** A3
**I want** to keep several instances running
**so that** I can compare them, while understanding the consequences.

Tags: `@FR-RUN-030` `@FR-RUN-090` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «Грабли»

**Preconditions**
- One instance is already running.

**Acceptance criteria**
- **AC-1.** When a second instance is launched the user is warned that there may not be enough VRAM.
- **AC-2.** The warning offers to stop the running instance or to launch anyway.
- **AC-3.** When launched anyway, the second instance receives a separate port.
- **AC-4.** Both instances are available for switching between.
- **AC-5.** Stopping one does not affect the other.

**Negative and edge cases**
- **AC-6.** If the second instance crashes for lack of VRAM, the state reflects the crash and the log holds the reason.

---

### US-RUN-06 — A server crash

**As** a user
**I want** to learn at once that the server has crashed
**so that** I do not wait for nothing and can understand the reason.

Tags: `@FR-RUN-060` `@phase-2` `@area-run`
Rationale: `PLAN.md` → «supervise/ + process.rs»

**Preconditions**
- The instance was running and its process finished on its own.

**Acceptance criteria**
- **AC-1.** The state changes to crashed.
- **AC-2.** This is reported regardless of which section of the app is open.
- **AC-3.** The last lines of the log are available without further actions.
- **AC-4.** The user can launch the instance again.
- **AC-5.** The port is released.

---

### US-RUN-07 — The server restarting itself

**As** a user who installed a custom node through ComfyUI-Manager
**I want** the app to understand that the server restarted itself
**so that** it does not look like a breakage.

Tags: `@FR-RUN-100` `@phase-4` `@area-run`
Rationale: `PLAN.md` → «Грабли»

**Preconditions**
- The instance is running, and an operation that restarts the server has been
  performed inside it.

**Acceptance criteria**
- **AC-1.** The app recognises that the server is available again on the same port and does not count this as a crash.
- **AC-2.** The user is told that the server restarted outside the app's control.
- **AC-3.** Reconnecting to the restarted server is offered.
- **AC-4.** After reconnecting, work with the instance continues as usual.

**Negative and edge cases**
- **AC-5.** If the server did not come up within a short waiting period, the case is treated as a crash per `US-RUN-06`.
- **AC-6.** Any limitations on controlling such a server are named to the user explicitly.

---

### US-RUN-08 — Closing the app while servers are running

**As** a user
**I want** to control what happens to the running servers when I close the app
**so that** I do not lose work or leave the VRAM occupied.

Tags: `@FR-RUN-080` `@phase-4` `@area-run`
Rationale: `PLAN.md` → «Грабли»

**Preconditions**
- At least one instance is running and the user is closing the app.

**Acceptance criteria**
- **AC-1.** The user is warned that there are running servers, and they are listed.
- **AC-2.** Stopping everything and quitting, or collapsing the app and leaving the servers running, are both offered.
- **AC-3.** When quitting is chosen, every server is stopped and the ports are released.
- **AC-4.** When collapsing, the app stays available and the servers keep running.

**Negative and edge cases**
- **AC-5.** A crash of the app itself leaves no ComfyUI servers running.
- **AC-6.** Launching the app again does not create a second copy of it but shows the one already open.
