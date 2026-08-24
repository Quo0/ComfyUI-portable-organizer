---
layout: home

hero:
  name: ComfyUI Portable Organizer
  text: Varias instalaciones portables, una sola ventana
  tagline: Un registro de tus instalaciones portables de ComfyUI, un lanzador que elige el puerto por ti y el propio ComfyUI como pestaña dentro de la aplicación — más una única carpeta de modelos compartida en lugar de cinco copias del mismo checkpoint.
  actions:
    - theme: brand
      text: Descargar para Windows
      link: /es/download
    - theme: alt
      text: Leer la guía
      link: /es/guide/
    - theme: alt
      text: GitHub
      link: https://github.com/Quo0/ComfyUI-portable-organizer

features:
  - title: Todas las instalaciones en una lista
    details: Señala una carpeta portable y la aplicación lee la versión de ComfyUI, la de Python y los scripts de arranque. Nada dentro de la carpeta se modifica.
    link: /es/guide/add-build
  - title: ComfyUI dentro de la ventana
    details: Cada instalación en marcha tiene su pestaña. Cambiar entre dos es un clic, y el navegador no se abre a tus espaldas.
    link: /es/guide/profiles
  - title: Una sola carpeta de modelos
    details: Varias instalaciones apuntando a la misma carpeta de modelos mediante el propio extra_model_paths.yaml de ComfyUI. Un checkpoint de 20 GB deja de ser cinco checkpoints de 20 GB.
    link: /es/guide/shared-models
  - title: Una biblioteca de flujos fuera de las instalaciones
    details: Los grafos que de verdad usas viven en una sola carpeta, y ves a qué instalación le faltan qué nodos antes de añadirle uno.
    link: /es/guide/workflows
  - title: Tus archivos siguen siendo tuyos
    details: Los modelos, los flujos y las propias instalaciones no se borran nunca — ni siquiera al desinstalar la aplicación. Tres operaciones tocan tus archivos, y cada una empieza con un clic tuyo.
    link: /es/guide/uninstall
  - title: Solo Windows, y a propósito
    details: Las instalaciones portables de ComfyUI son cosa de Windows. El instalador no necesita permisos de administrador y se instala en tu perfil de usuario.
    link: /es/guide/install-app
---

## Qué problema resuelve

La gente acaba con varias instalaciones portables de ComfyUI por un buen
motivo: los nodos personalizados entran en conflicto entre sí, y actualizar
una instalación rompe otra. Lo que queda es una carpeta llena de archivos
`.bat`, sin lista de qué hay instalado dónde, sin control sobre los
puertos, sin acceso al registro de arranque y con una pestaña de navegador
por instalación.

Esta aplicación es la envoltura que faltaba alrededor de esas carpetas.
No sustituye a ComfyUI, no lo parchea y no gestiona nodos personalizados:
para eso está ComfyUI-Manager.

## Qué no hace

- No edita grafos de flujos de trabajo ni genera vistas previas de ellos.
- No instala ni actualiza nodos personalizados.
- No cambia los ajustes de tema e idioma del propio ComfyUI.
- Nunca comparte `custom_nodes` entre instalaciones. Eso echaría por tierra
  la razón misma por la que están separadas.
