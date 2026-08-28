# The traceability matrix

Ties requirements, stories, implementation phases and sections of the plan
together. It exists for three questions: what is already covered by
requirements, what is planned for the nearest phase, and where to look for the
technical reasoning.

The sections of the plan are named here by title and live in `plan/`. The
section table is in `PLAN.md`, and the way in is always through it. The phases
are in `plan/phases.md`. Both are quoted in Russian because both documents are
still in Russian; the phase letters `4а` and `4в` are part of those names.

Status refers to the **implementation**, not to writing the specification.
Every requirement below has been described.

**The statuses lag behind the code.** The column was filled in while the
specifications were being written and has since been updated only for the areas
`EP-SHARED` (phase 2.5) and `EP-WF` (phase 2.6). Phases 0, 0.5, 1, 1.5 and 2
are closed — their requirements are implemented, but here they are still marked
"Not started". This is worth sorting out once and row by row rather than in
passing: a status set without looking is worse than no status at all.

---

## Requirements by area

### EP-ONB — Onboarding

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-ONB-010` | `US-ONB-01`, `US-ONB-02`, `US-ONB-03` | 1 | Навигация и структура приложения | Not started |
| `FR-ONB-020` | `US-ONB-04` | 1 | Навигация и структура приложения | Not started |
| `FR-ONB-030` | `US-ONB-01` | 1 | Экраны | Not started |
| `FR-ONB-040` | `US-ONB-01` | 0.5 | Навигация и структура приложения | Not started |
| `FR-ONB-050` | `US-ONB-02` | 1.5 | Мастер установки | Not started |

### EP-REG — The instance registry

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-REG-010` | `US-REG-01` | 1 | discovery/windows_portable.rs | Not started |
| `FR-REG-020` | `US-REG-01` | 1 | Дисциплина хранения данных | Not started |
| `FR-REG-030` | `US-REG-01`, `US-REG-03` | 1 | discovery/windows_portable.rs | Not started |
| `FR-REG-040` | `US-REG-02` | 1 | discovery/windows_portable.rs | Not started |
| `FR-REG-050` | `US-REG-04` | 1 | Экраны | Not started |
| `FR-REG-060` | `US-REG-05` | 1 | Чеклист, Фаза 1 | Not started |
| `FR-REG-070` | `US-REG-03` | 1.5 | Мастер установки | Not started |
| `FR-REG-080` | `US-REG-03` | 1 | Грабли | Not started |
| `FR-REG-090` | `US-REG-01` | 1 | discovery/windows_portable.rs | Not started |

### EP-INST — The install wizard

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-INST-010` | `US-INST-01` | 1.5 | Мастер установки | Not started |
| `FR-INST-020` | `US-INST-01`, `US-INST-07` | 1.5 | Мастер установки | Not started |
| `FR-INST-030` | `US-INST-02`, `US-INST-08` | 1.5 | Мастер установки | Not started |
| `FR-INST-040` | `US-INST-02`, `US-INST-08` | 1.5 | Мастер установки | Not started |
| `FR-INST-050` | `US-INST-03` | 1.5 | Мастер установки | Not started |
| `FR-INST-060` | `US-INST-02` | 1.5 | Грабли | Not started |
| `FR-INST-070` | `US-INST-05` | 1.5 | Мастер установки | Not started |
| `FR-INST-080` | `US-INST-05` | 1.5 | Интерфейс: прогресс | Not started |
| `FR-INST-090` | `US-INST-04`, `US-INST-06`, `US-INST-07` | 1.5 | Мастер установки | Not started |
| `FR-INST-100` | `US-INST-06`, `US-INST-07` | 1.5 | Мастер установки | Not started |
| `FR-INST-110` | `US-INST-02` | 1.5 | Мастер установки | Not started |

### EP-RUN — Launching and the lifecycle

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-RUN-010` | `US-RUN-01` | 2 | Парсинг .bat | Not started |
| `FR-RUN-020` | `US-RUN-02` | 0 | Ключевые находки, факт 1 | Not started |
| `FR-RUN-030` | `US-RUN-02`, `US-RUN-05` | 2 | ports.rs | Not started |
| `FR-RUN-040` | `US-RUN-03` | 2 | supervise/ + process.rs | Not started |
| `FR-RUN-050` | `US-RUN-03` | 2 | supervise/ + process.rs | Not started |
| `FR-RUN-060` | `US-RUN-06` | 2 | supervise/ + process.rs | Not started |
| `FR-RUN-070` | `US-RUN-04` | 2 | supervise/ + process.rs | Not started |
| `FR-RUN-080` | `US-RUN-08` | 2, 4а | supervise/ + process.rs | Done |
| `FR-RUN-090` | `US-RUN-05` | 4а | Грабли | Done |
| `FR-RUN-100` | `US-RUN-07` | 4а | Грабли | Done |
| `FR-RUN-110` | `US-RUN-01` | 4а | Парсинг .bat | Done |
| `FR-RUN-120` | `US-RUN-02` | 2 | supervise/windows.rs | Not started |

### EP-TAB — Navigation and embedded tabs

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-TAB-010` | `US-TAB-01` | 0.5 | Навигация и структура приложения | Done |
| `FR-TAB-020` | `US-TAB-02` | 0, 3 | Ключевые находки, факт 3 | Done |
| `FR-TAB-030` | `US-TAB-03` | 2 | Навигация и структура приложения | Done |
| `FR-TAB-040` | `US-TAB-03`, `US-TAB-04` | 2 | Навигация и структура приложения | Done |
| `FR-TAB-050` | `US-TAB-02`, `US-TAB-04` | 3 | webview.rs — встраивание | Done |
| `FR-TAB-060` | `US-TAB-02` | 3 | Грабли | Done, verified only by hand |
| `FR-TAB-070` | `US-TAB-05` | 3 | Грабли | Done |
| `FR-TAB-080` | `US-TAB-06` | 3 | Экраны | Done |
| `FR-TAB-090` | `US-TAB-06` | 3 | Экраны | Done |
| `FR-TAB-100` | `US-TAB-01` | 0.5 | Навигация и структура приложения | Done |

### EP-SHARED — Shared model storage

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-SHARED-010` | `US-SHARED-01` | 2.5 | Общее хранилище моделей | Done |
| `FR-SHARED-020` | `US-SHARED-02` | 2.5 | Генерация YAML | Done |
| `FR-SHARED-030` | `US-SHARED-03`, `US-SHARED-07` | 2.5 | Два режима применения | Done |
| `FR-SHARED-040` | `US-SHARED-03` | 2.5 | Что выяснено из исходников | Done |
| `FR-SHARED-050` | `US-SHARED-04` | 2.5 | Два режима применения | Done |
| `FR-SHARED-060` | `US-SHARED-05` | 2.5 | Два режима применения | Done |
| `FR-SHARED-070` | `US-SHARED-06`, `US-SHARED-07` | 2.5 | Два режима применения | Done |
| `FR-SHARED-080` | `US-SHARED-02` | 2.5 | Общее хранилище моделей | Done |
| `FR-SHARED-090` | `US-SHARED-08` | 2.5 | Грабли | Done |
| `FR-SHARED-100` | `US-SHARED-03` | 2.5 | Грабли | Done |
| `FR-SHARED-110` | `US-SHARED-09` | 4а | Чеклист, Фаза 4а | Done |
| `FR-SHARED-120` | `US-SHARED-01` | 2.5 | Модель данных | Done |
| `FR-SHARED-130` | `US-SHARED-10` | 2.5 | Общее хранилище моделей | Done |
| `FR-SHARED-140` | `US-SHARED-10`, `US-SHARED-11` | 2.5 | Общее хранилище моделей | Done |

### EP-WF — The workflow library

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-WF-010` | `US-WF-01` | 2.6 | Библиотека воркфлоу | Done |
| `FR-WF-020` | `US-WF-02` | 2.6 | Хранилище | Done |
| `FR-WF-030` | `US-WF-02` | 2.6 | Хранилище | Done |
| `FR-WF-040` | `US-WF-03` | 2.6 | Наполнение библиотеки | Done |
| `FR-WF-050` | `US-WF-04` | 2.6 | Добавление в инстанс | Done |
| `FR-WF-060` | `US-WF-04` | 2.6 | Добавление в инстанс | Done |
| `FR-WF-070` | `US-WF-05` | 2.6 | Проверка совместимости | Done |
| `FR-WF-080` | `US-WF-05` | 2.6 | Проверка совместимости | Done |
| `FR-WF-090` | `US-WF-06` | 2.6 | Хранилище | Done |
| `FR-WF-100` | `US-WF-07` | 2.6 | Добавление в инстанс | Done |
| `FR-WF-110` | `US-WF-06` | 2.6 | Грабли | Done |

### EP-UI — Appearance, language, notifications

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-UI-010` | `US-UI-01` | 0.5 | Интерфейс: тема | Not started |
| `FR-UI-020` | `US-UI-01` | 0.5 | Интерфейс: тема | Not started |
| `FR-UI-030` | `US-UI-02` | 0.5 | Интерфейс: язык | Not started |
| `FR-UI-040` | `US-UI-02` | 0.5 | Интерфейс: язык | Not started |
| `FR-UI-050` | `US-UI-02` | 0.5 | Интерфейс: язык | Not started |
| `FR-UI-060` | `US-UI-03` | 0.5 | Интерфейс: уведомления | Not started |
| `FR-UI-070` | `US-UI-03` | 0.5 | Интерфейс: уведомления | Not started |
| `FR-UI-080` | `US-UI-04` | 2 | Интерфейс: уведомления | Not started |
| `FR-UI-090` | `US-UI-05` | 1.5 | Интерфейс: прогресс | Not started |
| `FR-UI-100` | `US-UI-02` | 0.5 | Интерфейс: язык | Not started |
| `FR-UI-110` | `US-UI-02` | 0.5 | Интерфейс: язык | Not started |

### EP-DATA — Data storage and removal

| FR | Stories | Phase | Plan section | Status |
|---|---|---|---|---|
| `FR-DATA-010` | `US-DATA-01` | 0 | Дисциплина хранения данных | Not started |
| `FR-DATA-020` | `US-DATA-03` | 2.5, 2.6 | Дисциплина хранения данных | Not started |
| `FR-DATA-030` | `US-DATA-03` | 0 | Дисциплина хранения данных | Not started |
| `FR-DATA-040` | `US-DATA-02` | 4в | Дисциплина хранения данных | Not started |
| `FR-DATA-050` | `US-DATA-02` | 4в | Дисциплина хранения данных | Not started |
| `FR-DATA-060` | `US-DATA-03` | 4в | Дисциплина хранения данных | Not started |
| `FR-DATA-070` | `US-DATA-01` | 4а | Дисциплина хранения данных | Done |
| `FR-DATA-080` | `US-DATA-02` | 4в | Дисциплина хранения данных | Not started |
| `FR-DATA-090` | `US-DATA-01` | 2.5, 2.6, 4а | Дисциплина хранения данных | Done |
| `FR-DATA-100` | `US-DATA-04` | 4в | Выпуск | Done |
| `FR-DATA-110` | `US-DATA-04` | 4в | Выпуск | Done |
| `FR-DATA-120` | `US-DATA-04` | 4в | Выпуск | Done |

---

## End-to-end journeys and stories

| Journey | Stories |
|---|---|
| `J-01` A clean machine | `US-ONB-01`, `US-ONB-02`, `US-INST-01`…`US-INST-06`, `US-RUN-02`, `US-RUN-03`, `US-TAB-02` |
| `J-02` There is already a folder | `US-ONB-01`, `US-ONB-03`, `US-REG-01`, `US-REG-02`, `US-RUN-01`, `US-RUN-02`, `US-TAB-02` |
| `J-03` Shared models | `US-SHARED-01`…`US-SHARED-04`, `US-SHARED-06`…`US-SHARED-09` |
| `J-04` A new version alongside | `US-INST-07`, `US-INST-02`, `US-INST-04`, `US-INST-05`, `US-REG-03`, `US-RUN-02` |
| `J-05` Carrying a workflow over | `US-WF-01`…`US-WF-05`, `US-TAB-02` |
| `J-06` Uninstalling | `US-DATA-01`, `US-DATA-02`, `US-DATA-03`, `US-DATA-04` |

---

## Coverage of the implementation phases

| Phase | Covered by stories | Comment |
|---|---|---|
| 0 — The repository and the spike | Partly | A technical spike; the user behaviour is checked through `FR-RUN-020`, `FR-TAB-020` |
| 0.5 — The interface skeleton | Yes | `EP-UI`, `US-TAB-01` |
| 1 — The instance registry | Yes | `EP-REG`, `EP-ONB` |
| 1.5 — The install wizard | Yes | `EP-INST` |
| 2 — Launching and the lifecycle | Yes | `EP-RUN`, `US-TAB-03`, `US-TAB-04` |
| 2.5 — Shared model storage | Yes | `EP-SHARED` |
| 2.6 — The workflow library | Yes | `EP-WF` |
| 3 — Embedded tabs | Yes | `US-TAB-02`, `US-TAB-05`, `US-TAB-06` |
| 4а — Polish | Yes | `US-DATA-01`, `US-RUN-07`, `US-RUN-08`, `US-SHARED-09` |
| 4в — Packaging and release | Yes | `US-DATA-02`, `US-DATA-03`, `US-DATA-04` |
| 5 — The documentation site | No | Not user behaviour of the app; derived from `journeys.md` |

---

## Plan sections outside the coverage

They describe internal mechanics not directly observable by the user. We write
no requirements for them; their correctness is checked through the requirements
that rest on them.

| Section | What checks it |
|---|---|
| Структура репозитория | — |
| Стек | — |
| supervise/ + process.rs | `FR-RUN-040`…`FR-RUN-080` |
| ports.rs | `FR-RUN-030` |
| installer.rs | `EP-INST` as a whole |
| Кроссплатформенность | `NFR-080` |
| Дисциплина z-order | `NFR-300` |
