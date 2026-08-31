# Coverage

Eighteen screens cover all six end-to-end scenarios from `specs/journeys.md`.
Not shown are the screens that only appear in exceptional situations: resolving
the conflict when a build already holds someone else's config file, and the
duplicate models report — both belong to later phases.

Every component is taken from the shared library rather than drawn again: both
pages share one style file. So editing a component in the library changes it on
every screen automatically.
