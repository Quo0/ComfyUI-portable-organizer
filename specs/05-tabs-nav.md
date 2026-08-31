# EP-TAB — Navigation and embedded tabs

The app's main distinguishing feature: ComfyUI opens inside the window rather
than in a browser. The structure follows from that — a permanently visible rail
on the left and a changing content area, in which ComfyUI itself lives for a
running instance.

The embedded tab is not a picture and not a cut-down version. It is a fully
featured ComfyUI with all of its capabilities, including dragging files onto
the canvas and following external links.

## Functional requirements

| ID | Requirement |
|---|---|
| `FR-TAB-010` | Navigation between the sections is always visible, including on the screen of a running instance |
| `FR-TAB-020` | The ComfyUI interface is shown inside the app's window |
| `FR-TAB-030` | Running instances are available for switching between in a single action |
| `FR-TAB-040` | Leaving for another section does not interrupt an instance's work |
| `FR-TAB-050` | The ComfyUI area follows changes in the window's size and in the navigation |
| `FR-TAB-060` | Dragging files onto the ComfyUI canvas works |
| `FR-TAB-070` | External links open in the system browser |
| `FR-TAB-080` | The tools for working with an instance are available without leaving its screen |
| `FR-TAB-090` | An instance can be opened in an ordinary browser |
| `FR-TAB-100` | The rail collapses down to icons, and the state is remembered |

---

### US-TAB-01 — Navigating between the sections

**As** a user
**I want** to move quickly between parts of the app
**so that** I do not lose context.

Tags: `@FR-TAB-010` `@FR-TAB-100` `@phase-0.5` `@area-tab`

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** The app's sections are available from a permanently visible menu: instances, installation, settings, information about the app. The workflow library is a settings section: it is a folder outside the builds, like the shared models folder, and it is configured in the same place where it is shown.
- **AC-2.** The current section is visually distinct from the rest.
- **AC-3.** The menu is visible on any screen, including the screen of a running instance.
- **AC-4.** The menu can be collapsed to a compact form and expanded back.
- **AC-5.** The collapsed state is kept between launches of the app.
- **AC-6.** In the compact form the purpose of each item stays recognisable.

**Negative and edge cases**
- **AC-7.** The section labels are not truncated and do not break the layout in any of the supported languages.

---

### US-TAB-02 — ComfyUI inside the app's window

**As** a user
**I want** to work with ComfyUI right inside the app
**so that** I do not have to hunt for the right tab among the others in the
browser.

Tags: `@FR-TAB-020` `@FR-TAB-050` `@FR-TAB-060` `@phase-3` `@area-tab`

**Preconditions**
- The instance has been launched and has reached readiness.

**Acceptance criteria**
- **AC-1.** The ComfyUI interface occupies the whole content area of the window.
- **AC-2.** The interface is fully featured: every capability available in a browser is available here.
- **AC-3.** Dragging an image or a workflow file onto the canvas loads it, as in a browser.
- **AC-4.** The ComfyUI area follows changes in the window's size without gaps or overlaps.
- **AC-5.** Collapsing and expanding the menu changes the size of the ComfyUI area accordingly.
- **AC-6.** Saving images and exporting from the ComfyUI interface work.

**Negative and edge cases**
- **AC-7.** An error loading the interface is explained to the user and offers to retry.
- **AC-8.** The elements of our interface do not cover the ComfyUI area but live in the places set aside for them.

---

### US-TAB-03 — Switching between running instances

**As** A3, working with two builds
**I want** to switch between them instantly
**so that** I can compare the results.

Tags: `@FR-TAB-030` `@FR-TAB-040` `@phase-2` `@area-tab`

**Preconditions**
- More than one instance is running.

**Acceptance criteria**
- **AC-1.** Every running instance is listed in the permanently visible menu.
- **AC-2.** Each of them is distinguishable by name and accent colour.
- **AC-3.** Switching is done in a single action.
- **AC-4.** The state of ComfyUI is preserved across a switch: an unsaved graph is not lost.
- **AC-5.** Each instance's state is reflected right in the menu.
- **AC-6.** An instance that has crashed is noticeable in the menu regardless of which section is open.

**Negative and edge cases**
- **AC-7.** A stopped instance disappears from the list of running ones but stays in the instances section.

---

### US-TAB-04 — Leaving for another section and coming back

**As** a user
**I want** to visit the settings without interrupting ComfyUI's work
**so that** I do not lose an unsaved graph.

Tags: `@FR-TAB-040` `@FR-TAB-050` `@phase-2` `@area-tab`

**Preconditions**
- The instance is running and its interface is open.

**Acceptance criteria**
- **AC-1.** Moving to another section hides the ComfyUI interface but does not interrupt the server.
- **AC-2.** Coming back shows the interface in the same state it was left in.
- **AC-3.** On the way back the ComfyUI area occupies the right place, even if the window's size changed while it was hidden.
- **AC-4.** The screens of the other sections are meanwhile displayed in full and covered by nothing.

---

### US-TAB-05 — External links

**As** a user who clicked a link inside ComfyUI
**I want** it to open in my browser
**so that** I do not end up in a window with no address bar and no back
button.

Tags: `@FR-TAB-070` `@phase-3` `@area-tab`

**Preconditions**
- The interface of a running instance is open.

**Acceptance criteria**
- **AC-1.** A link leading beyond the local server opens in the default browser.
- **AC-2.** The embedded area meanwhile stays on the ComfyUI page.
- **AC-3.** Navigation inside ComfyUI itself stays inside the embedded area.

**Negative and edge cases**
- **AC-4.** External authorisation flows that require a browser are carried out in the browser and do not block the app.

---

### US-TAB-06 — The instance tools

**As** a user working in ComfyUI
**I want** to manage the instance without leaving its screen
**so that** I can stop or restart the server in one step.

Tags: `@FR-TAB-080` `@FR-TAB-090` `@phase-3` `@area-tab`

**Preconditions**
- The interface of a running instance is open.

**Acceptance criteria**
- **AC-1.** Stopping and restarting the instance are available.
- **AC-2.** Viewing the logs of the current run is available.
- **AC-3.** Opening the folder with the generation results is available.
- **AC-4.** Opening this same instance in an ordinary browser is available.
- **AC-5.** The tools do not cover ComfyUI's working area.

**Negative and edge cases**
- **AC-6.** Opening it in a browser does not interrupt the embedded tab — both work with the same server.
