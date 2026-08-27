---
layout: home

hero:
  name: ComfyUI Portable Organizer
  tagline: Managing many portable builds, made easy
  image:
    src: /logo.svg
    alt: ComfyUI Portable Organizer
    width: 340
    height: 340
  actions:
    - theme: brand
      text: Download for Windows
      link: /download
    - theme: alt
      text: Read the guide
      link: /guide/
    - theme: alt
      text: GitHub
      link: https://github.com/Quo0/ComfyUI-portable-organizer

features:
  - title: Every build in one list
    details: Point the app at a portable folder and it reads the ComfyUI version, the Python version and the launch scripts. Nothing inside the folder is modified.
    link: /guide/add-build
  - title: ComfyUI inside the window
    details: Each running build gets a tab. Switching between two builds is one click, and the browser never opens behind your back.
    link: /guide/profiles
  - title: A shared models folder
    details: Point several builds at the same models folder through ComfyUI's own extra_model_paths.yaml. A 20 GB checkpoint stops being five 20 GB checkpoints.
    link: /guide/shared-models
  - title: A workflow library outside the builds
    details: Keep the graphs you actually use in one folder, and see which build is missing which nodes before you add one.
    link: /guide/workflows
  - title: Your files stay yours
    details: Models, workflows and the builds themselves are never deleted — not even when you uninstall the app. Three operations touch your files, each one starts with your click.
    link: /guide/uninstall
  - title: Install into several folders at once
    details: The ComfyUI archive you downloaded is unpacked once and laid out into as many folders as you need — each with its own name, description and colour. The wizard checks the free space and the path length for you, and wires the new builds straight into your shared models folder and workflow library.
    link: /guide/install-from-archive
---

## What problem it solves

People end up with several portable ComfyUI installs for a good reason:
custom nodes conflict with each other, and updating one build breaks
another. What that leaves you with is a folder of `.bat` files, no list of
what is installed where, no control over ports, no visibility into the
startup log, and a browser tab per build.

This app is the missing shell around those folders. It does not replace
ComfyUI, does not patch it, and does not manage custom nodes — that is
what ComfyUI-Manager is for.

## Why several portable builds instead of Comfy Desktop

The hard part of ComfyUI is not the models, it is **custom node
dependencies.** Two nodes can ask for different versions of the same
Python library, so installing or updating one of them breaks the other.
ComfyUI's own developers run into this often enough to discuss it in the
open: the dependency sets of different custom nodes are simply not always
compatible. Updates to ComfyUI itself and to its Python dependencies can
break an environment that worked yesterday — which is why, on a serious
project, you want to freeze the build that works and experiment somewhere
else.

Portable answers that. A portable build is a self-contained folder with
its own `python_embeded`, its own ComfyUI and its own dependencies. Keep
as many as you need: a stable one for the workflows you deliver, an
experimental one for new versions, a separate one for that node set with
opinions about torch. Break one and the rest never find out.

What portable does not answer is the bookkeeping. Several folders, a
`.bat` file each, ports to keep apart, windows to keep track of, models
and workflows scattered across all of it — that turns into manual routine
fast.

That gap is what this app fills. It keeps what portable is good at —
isolation — and removes what it is bad at. Every build sits in one list:
start and stop them, assign ports, open them in tabs, share one models
folder and one workflow library. The builds themselves are left alone —
your Python environments, your custom nodes and your `.bat` files stay
entirely independent.

[Comfy Desktop](https://comfy.org/download) tries to give you convenience
and environment management in one package. This app leaves the
environment to you and adds the convenience on top of portable.

## What it does not do

- It does not edit workflow graphs or render previews of them.
- It does not install or update custom nodes.
- It does not change ComfyUI's own theme or language settings.
- It never shares `custom_nodes` between builds. That would undo the whole
  reason the builds are separate.
