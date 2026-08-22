# Updating the app

The app checks for a new version at startup and shows what it found under
**About**, together with the release notes for that version.

## The check is the only thing that leaves your computer

It sends the current version number to GitHub and asks whether a newer
release exists. Nothing else about you, your builds or your files is ever
sent anywhere.

It can be turned off: **About → Check for updates on startup**. Turned off,
it does not run at all — there is no hidden second schedule. You can still
press **Check now** whenever you like.

With no network, nothing happens and nothing is reported. A network error
you did not ask for looks like a broken app, and the app is not broken;
your connection is missing.

## Nothing installs without you

The update is downloaded and installed only after you press the button. Its
signature is verified before anything is installed, and an update that does
not verify is not installed — you are told why. That signature is a
`minisign` key pair whose public half is compiled into the app; it has
nothing to do with the [SmartScreen warning](/guide/smartscreen), which is
a different mechanism entirely.

## Running builds are your decision

Installing an update closes the app — that is how Windows installers work —
and every running ComfyUI server goes with it, because they are its child
processes.

So if anything is running, the installation does not start. You get the
list of running builds and two options:

- **Stop them and install** — the servers are stopped properly, then the
  update goes ahead;
- **Postpone until next launch** — nothing happens now, and the same update
  is offered again the next time you start the app.

Your generation queue is worth more than being one version behind for an
hour.

## Your data survives the update

Installing a new version over an old one runs in update mode, which skips
the data cleanup entirely. Settings, the build registry, shared-folder
connections and the workflow library are all where you left them.

Manual updates work the same way: download the new installer and run it
over the old version.
