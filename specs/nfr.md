# Non-functional requirements

Properties of the app as a whole rather than of individual scenarios. Every
number lives here: if an acceptance criterion says "quickly" or "does not
block", the specifics belong here.

Some of the values are taken from measurements on a real ComfyUI installation
— they are marked as measured and must not be changed without a new
measurement.

---

## Performance

| ID | Requirement |
|---|---|
| `NFR-010` | The app opens and shows the list of instances in no more than two seconds on an ordinary disk |
| `NFR-020` | The first log lines of an instance being launched appear within five seconds of the launch beginning |
| `NFR-030` | A cold start of an instance lasting up to five minutes counts as ordinary; breaking the wait off earlier is not acceptable |
| `NFR-040` | The server's readiness is detected within a second of it actually being ready |
| `NFR-050` | Unpacking an archive into several folders in one run is noticeably faster than the same number of separate installations |
| `NFR-060` | No operation blocks the interface: the user can move to another section during any long-running work |
| `NFR-070` | Switching between running instances happens without a noticeable delay and without reloading their interfaces |

**Measured on a real installation:** walking the tree of an unpacked build of
52 GB takes more than five minutes. Hence `NFR-060` and the requirement for
computing the size on disk asynchronously in `US-REG-03`.

---

## Platform constraints

| ID | Requirement |
|---|---|
| `NFR-080` | The target platform is Windows. Other systems are not supported and not tested |
| `NFR-090` | The app works with portable ComfyUI builds; other ways of installing ComfyUI are out of scope |
| `NFR-100` | Installing and uninstalling the app do not require administrator rights |
| `NFR-110` | The app works without a network connection; the network is needed only by ComfyUI itself |
| `NFR-120` | Paths with spaces and non-Latin characters are supported on the same footing as the rest |
| `NFR-130` | The nesting depth of files inside a build must not cause the extraction to fail |

**Measured:** the deepest file in a portable build archive has a path about 206
characters long relative to the archive's root. Against the system limit of 260
characters that leaves fewer than sixty characters for the path to the
destination folder. Hence the warning in `US-INST-02/AC-7` and the requirement
`NFR-130`.

---

## Reliability

| ID | Requirement |
|---|---|
| `NFR-140` | A crash of the app leaves no ComfyUI processes running |
| `NFR-150` | Interrupting a long operation does not leave the system in an intermediate state |
| `NFR-160` | An unfinished installation cannot be taken for a working instance |
| `NFR-170` | Damage to the app's housekeeping information does not lead to losing the user's content |
| `NFR-180` | The absence or unavailability of an external folder — the shared root, the library, an instance's folder — does not put the app out of action |
| `NFR-190` | The log of an instance being launched is kept in a volume sufficient to diagnose a problem and does not grow without bound |
| `NFR-200` | Only one copy of the app runs at a time |

---

## Localisation

| ID | Requirement |
|---|---|
| `NFR-210` | English, Russian, Simplified Chinese and Spanish are supported |
| `NFR-220` | English is the source language; the rest are translated from it |
| `NFR-230` | A divergence in the set of translations between languages must not reach a build |
| `NFR-240` | No interface text is left untranslated in a supported language |
| `NFR-250` | The layout withstands the longest string variants among the supported languages |
| `NFR-260` | Word forms in counted phrases, numbers and dates obey the rules of the chosen language |
| `NFR-270` | Right-to-left languages are not supported and are out of scope |

---

## Styling

| ID | Requirement |
|---|---|
| `NFR-280` | Every screen is legible in the light and the dark theme |
| `NFR-290` | Changing the theme and the language does not require restarting the app |
| `NFR-300` | The app's interface elements do not cover ComfyUI's working area |

---

## Behaviour with large amounts of data

The window has a fixed height and there can be any amount of data: eight
instances, two hundred workflows, twenty-five model categories, thousands of
log lines.

| ID | Requirement |
|---|---|
| `NFR-420` | A screen's controls — the header, the primary actions, the footer with the navigation buttons — stay reachable without scrolling for any amount of data |
| `NFR-430` | The launch console follows the last lines until the user scrolls up; after that the following pauses and resumes on an explicit action |
| `NFR-440` | The area of the embedded ComfyUI tab does not take part in scrolling: its rectangle does not shift relative to the window |
| `NFR-450` | The list of running instances in the navigation scrolls inside itself without pushing out the app's sections |
| `NFR-460` | The boundaries of a scroll area are discernible: the user can see that the content continues |

A note on `NFR-420`: a wizard footer that has slid out of sight makes the
wizard impassable — the "Next" button becomes unreachable. A details panel that
has slid out of sight in the library makes the selected workflow invisible.
That is why only the data area scrolls.

A note on `NFR-440`: the position of the native window holding the ComfyUI
interface is set by the app in the window's coordinates. Scrolling the
container would shift the markup but not the window itself, and the two would
drift apart.

---

## The attitude to other people's data

| ID | Requirement |
|---|---|
| `NFR-310` | The app does not change the contents of instance folders without an explicit command from the user |
| `NFR-320` | The app does not change ComfyUI's settings inside instances |
| `NFR-330` | Any write into an instance's folder is reversible and named to the user |
| `NFR-340` | The app does not delete models, workflows or ComfyUI builds under any circumstances, including its own removal |
| `NFR-350` | The app does not send data beyond the user's computer, apart from the update check — see `NFR-355` |
| `NFR-355` | The only outgoing request is the update check: only the current version number leaves, it runs on a schedule or on a button, and it can be switched off in the settings |

---

## Security

| ID | Requirement |
|---|---|
| `NFR-360` | The app does not weaken ComfyUI's protective mechanisms for its own convenience |
| `NFR-370` | The local ComfyUI server does not become more reachable from outside because of the app's actions |
| `NFR-380` | Navigation beyond the local server happens in the system browser rather than inside the app |

A note on `NFR-360`: ComfyUI has built-in protection against requests from
other origins. There is a flag that switches it off entirely, and it would have
made embedding the interface easier — but it would open the server to requests
from any site open in the user's browser. The app manages without it.

---

## Updating the app

| ID | Requirement |
|---|---|
| `NFR-470` | An update is not installed without the user's explicit consent |
| `NFR-480` | Before the installation it is checked whether there are running instances; the user decides what to do with them |
| `NFR-490` | The update's integrity is verified by signature before the installation; a mismatch means it is not installed |
| `NFR-500` | An unavailable network does not get in the way of the work and does not show errors by itself |
| `NFR-510` | An update keeps the settings and the instance registry |

A note on `NFR-480`: installing an update on Windows closes the app by force,
and the child ComfyUI processes are terminated along with it. An update in the
middle of a generation would cost the user their queue and several minutes for
a new cold start.

---

## Compatibility

| ID | Requirement |
|---|---|
| `NFR-390` | The app works with different versions of ComfyUI without depending on a hardcoded list of model categories |
| `NFR-400` | The appearance of new model types in ComfyUI does not require an update to the app |
| `NFR-410` | A non-standard location of user data inside a build is taken into account rather than assumed by default |
