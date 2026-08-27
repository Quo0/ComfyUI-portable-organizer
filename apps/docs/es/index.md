---
layout: home

hero:
  name: ComfyUI Portable Organizer
  tagline: Gestionar varias instalaciones portables, sin complicaciones
  image:
    src: /logo.svg
    alt: ComfyUI Portable Organizer
    width: 340
    height: 340
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
  - title: Una carpeta de modelos compartida
    details: Varias instalaciones apuntando a la misma carpeta de modelos mediante el propio extra_model_paths.yaml de ComfyUI. Un checkpoint de 20 GB deja de ser cinco checkpoints de 20 GB.
    link: /es/guide/shared-models
  - title: Una biblioteca de flujos fuera de las instalaciones
    details: Los grafos que de verdad usas viven en una sola carpeta, y ves a qué instalación le faltan qué nodos antes de añadirle uno.
    link: /es/guide/workflows
  - title: Tus archivos siguen siendo tuyos
    details: Los modelos, los flujos y las propias instalaciones no se borran nunca — ni siquiera al desinstalar la aplicación. Tres operaciones tocan tus archivos, y cada una empieza con un clic tuyo.
    link: /es/guide/uninstall
  - title: Instalación en varias carpetas a la vez
    details: El archivo de ComfyUI que descargaste se descomprime una sola vez y se reparte por tantas carpetas como necesites — cada una con su nombre, su descripción y su color. El asistente comprueba por ti el espacio libre y la longitud de la ruta, y conecta las instalaciones nuevas a tu carpeta de modelos compartida y a la biblioteca de flujos.
    link: /es/guide/install-from-archive
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

## Por qué varias instalaciones portables y no Comfy Desktop

Lo difícil de ComfyUI no son los modelos, sino **las dependencias de los
nodos personalizados.** Dos nodos pueden pedir versiones distintas de la
misma biblioteca de Python, así que instalar o actualizar uno rompe el
otro. A los propios desarrolladores de ComfyUI les ocurre lo bastante a
menudo como para hablarlo en abierto: los conjuntos de dependencias de
distintos nodos personalizados sencillamente no siempre son compatibles.
Y las actualizaciones del propio ComfyUI y de sus dependencias de Python
rompen entornos que ayer funcionaban — por eso, en un proyecto serio,
quieres congelar la instalación que funciona y experimentar en otro sitio.

Portable responde justo a eso. Una instalación portable es una carpeta
autónoma con su propio `python_embeded`, su propio ComfyUI y sus propias
dependencias. Ten tantas como necesites: una estable para los flujos que
entregas, otra experimental para las versiones nuevas, otra aparte para ese
conjunto de nodos con opiniones sobre torch. Si una se rompe, las demás ni
se enteran.

A lo que portable no responde es al mantenimiento. Varias carpetas, un
archivo `.bat` en cada una, puertos que separar, ventanas que vigilar,
modelos y flujos repartidos por todo ello: se convierte en rutina manual
enseguida.

Ese hueco es el que cubre esta aplicación. Conserva aquello en lo que
portable es bueno — el aislamiento — y quita aquello en lo que es malo.
Todas las instalaciones en una sola lista: arrancarlas y pararlas,
asignarles puertos, abrirlas en pestañas, compartir una carpeta de modelos
y una biblioteca de flujos. Las instalaciones en sí quedan intactas: tus
entornos de Python, tus nodos personalizados y tus archivos `.bat` siguen
siendo completamente independientes.

[Comfy Desktop](https://comfy.org/download) intenta darte la comodidad y la
gestión del entorno en un mismo paquete. Esta aplicación te deja el entorno
a ti y añade la comodidad encima de portable.

## Qué no hace

- No edita grafos de flujos de trabajo ni genera vistas previas de ellos.
- No instala ni actualiza nodos personalizados.
- No cambia los ajustes de tema e idioma del propio ComfyUI.
- Nunca comparte `custom_nodes` entre instalaciones. Eso echaría por tierra
  la razón misma por la que están separadas.
