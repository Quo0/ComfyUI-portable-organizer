# EP-UI — Appearance, language and notifications

The app's styling, support for four languages, and the way the app reports the
results of operations.

A separate caveat about ComfyUI inside the embedded tab: it has theme and
language settings of its own, and the app **does not touch** them. A mismatch
between a dark shell and a light canvas is an accepted decision, not a defect:
we are not going to silently rewrite the user's settings in someone else's
application.

## Functional requirements

| ID | Requirement |
|---|---|
| `FR-UI-010` | Light, dark and system themes are supported |
| `FR-UI-020` | In system mode the app follows the Windows theme without a restart |
| `FR-UI-030` | The interface is available in English, Russian, Chinese and Spanish |
| `FR-UI-040` | The language is determined from the system one on the first launch and can be overridden by the user |
| `FR-UI-050` | A change of language applies without restarting the app |
| `FR-UI-060` | Both successful and unsuccessful operations are confirmed by a notification |
| `FR-UI-070` | An error message holds enough detail for diagnosis and can be copied |
| `FR-UI-080` | Events not caused by a user action are visible regardless of which section is open |
| `FR-UI-090` | Long operations show their progress in the section they belong to |
| `FR-UI-100` | The theme and language settings of ComfyUI inside the tab are not changed by the app |
| `FR-UI-110` | The interface texts are complete and correct in every supported language |

---

### US-UI-01 — Choosing the theme

**As** a user
**I want** to choose the app's styling
**so that** it does not hurt my eyes when I work at night.

Tags: `@FR-UI-010` `@FR-UI-020` `@phase-0.5` `@area-ui`

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** Three options are available: light, dark and follow the system.
- **AC-2.** Following the system is chosen by default.
- **AC-3.** A change of theme applies at once, without a restart.
- **AC-4.** The choice is kept between launches.
- **AC-5.** In follow-the-system mode, a change of the Windows theme changes the app's styling on the fly.
- **AC-6.** The window title bar is styled to match the chosen theme.
- **AC-7.** Every screen is legible in both themes: there are no elements that lose their contrast.

**Negative and edge cases**
- **AC-8.** An instance's accent colour stays distinguishable in both themes; when picking a colour the user is warned if it is not.
- **AC-9.** ComfyUI's styling inside the embedded tab does not change to follow our theme, and that is expected.

---

### US-UI-02 — Choosing the language

**As** a user whose native language is not English
**I want** to use the app in my own language
**so that** I do not have to decipher the wording.

Tags: `@FR-UI-030` `@FR-UI-040` `@FR-UI-050` `@FR-UI-110` `@phase-0.5` `@area-ui`

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** English, Russian, Chinese and Spanish are available.
- **AC-2.** On the first launch the system language is chosen if it is supported; otherwise English.
- **AC-3.** The user can change the language in the settings.
- **AC-4.** A change of language applies at once, without a restart.
- **AC-5.** The choice is kept between launches.
- **AC-6.** Every interface text is translated, error messages included.
- **AC-7.** Numbers, dates and file sizes are displayed by the rules of the chosen language.
- **AC-8.** Word forms in counted phrases agree with the number by the rules of the language.

**Negative and edge cases**
- **AC-9.** In no language are the labels truncated or the layout broken.
- **AC-10.** Chinese text is displayed with a proper font rather than substitute glyphs.
- **AC-11.** File paths, instance names and log contents are not translated.
- **AC-12.** ComfyUI's own language settings inside the embedded tab are not changed.

---

### US-UI-03 — Confirming the result of an operation

**As** a user
**I want** to see whether an operation succeeded
**so that** I do not have to guess whether the action worked.

Tags: `@FR-UI-060` `@FR-UI-070` `@phase-0.5` `@area-ui`

**Preconditions**
- The user performs an action: adds an instance, connects the shared models,
  adds a workflow.

**Acceptance criteria**
- **AC-1.** A successful completion is confirmed by a notification.
- **AC-2.** A success notification disappears by itself after a few seconds.
- **AC-3.** An unsuccessful completion is reported by a notification that does not disappear by itself.
- **AC-4.** The error message explains what happened, in the user's language.
- **AC-5.** The error details can be expanded and copied in full.
- **AC-6.** Several notifications in a row neither obscure each other nor fill the screen.
- **AC-7.** Repeated identical notifications are merged.

**Negative and edge cases**
- **AC-8.** Notifications do not cover ComfyUI's working area.
- **AC-9.** A message for an error that has no translation is still informative and holds a recognisable code.

---

### US-UI-04 — Events without the user's involvement

**As** a user working in ComfyUI
**I want** to learn that the server has crashed
**so that** I do not find out from a stalled generation.

Tags: `@FR-UI-080` `@phase-2` `@area-ui`

**Preconditions**
- The instance is running, and an event occurred without the user's
  involvement.

**Acceptance criteria**
- **AC-1.** A change in an instance's state is noticeable regardless of which section is open.
- **AC-2.** The state is visible in a permanently available part of the interface.
- **AC-3.** A crash is visually distinct from an ordinary stop.
- **AC-4.** The mark of the occurrence is kept until the user has seen it.
- **AC-5.** The details are available in the corresponding instance's log.

---

### US-UI-05 — The progress of long operations

**As** a user who started a long operation
**I want** to see that it is running
**so that** I can tell work apart from a freeze.

Tags: `@FR-UI-090` `@phase-1.5` `@area-ui`

**Preconditions**
- An operation taking a noticeable amount of time has been started.

**Acceptance criteria**
- **AC-1.** The operation's progress is shown in the same place it was started.
- **AC-2.** If the total amount of work is known, the fraction done is shown.
- **AC-3.** If the amount is unknown, the fact that work is happening is shown, without a false scale.
- **AC-4.** The interface stays responsive: the user can leave for another section.
- **AC-5.** The operation can be interrupted, if its nature permits.

**Negative and edge cases**
- **AC-6.** There is no permanent indicator for quick operations: an instantaneous action is not accompanied by a flickering bar.
