# Añadir una instalación

Ve a **Añadir**. Hay dos caminos: señalar una carpeta que ya tengas o
[descomprimir una instalación nueva desde un archivo](/es/guide/install-from-archive).

## Señalar una carpeta existente

Elige la **raíz** de la instalación portable — la carpeta que contiene
`python_embeded\` y `ComfyUI\`, no una de las dos:

```
ComfyUI_windows_portable\      ← esta
├─ python_embeded\
│  └─ python.exe
├─ ComfyUI\
│  └─ main.py
├─ run_nvidia_gpu.bat
├─ run_cpu.bat
└─ advanced\
   └─ run_nvidia_gpu_disable_api_nodes.bat
```

La aplicación comprueba que `python_embeded\python.exe` y
`ComfyUI\main.py` existen, lee la versión de ComfyUI de
`ComfyUI\comfyui_version.py`, le pregunta a `python.exe` por la suya y
recoge los archivos `.bat` como
[perfiles de arranque](/es/guide/profiles).

Después le das a la instalación un nombre, una descripción opcional, un
color de acento y un puerto preferido. Los cuatro se pueden cambiar
más tarde.

## Qué escribe la aplicación dentro de la carpeta

Por defecto, **nada**. El registro vive en la carpeta de datos de la propia
aplicación y solo recuerda dónde está cada instalación.

Dentro de una carpeta de instalación pueden llegar a escribirse exactamente
dos cosas, y cada una empieza con un clic tuyo:

- `ComfyUI\extra_model_paths.yaml`, y solo si eliges el
  [modo «archivo dentro de la instalación»](/es/guide/shared-models#dos-formas-de-aplicarlo)
  para los modelos compartidos;
- copias de los flujos que añades a esa instalación desde
  [la biblioteca](/es/guide/workflows).

Ambas figuran en la pantalla **Acerca de**, y ambas se quedan cuando
desinstalas la aplicación: están dentro de la instalación de otro, y
sacarlas de ahí no nos corresponde.

## Una carpeta que falta no es una instalación borrada

Si mueves o renombras la carpeta de una instalación, la entrada sigue en el
registro y se marca como no disponible. No desaparece en silencio: eso
parecería que la aplicación ha perdido tu instalación. Señálale la nueva
ubicación o elimina la entrada.

**Eliminar una entrada elimina la entrada.** La carpeta del disco no se
toca.

## Dos instalaciones de la misma versión, una al lado de otra

Nada te impide registrar el mismo archivo descomprimido dos veces, o dos
instalaciones que solo se diferencien en los nodos personalizados. Dales
nombres y colores de acento distintos: el color acompaña a la instalación
por el raíl, las tarjetas y la pestaña, y es la forma más rápida de
distinguir dos instalaciones parecidas.
