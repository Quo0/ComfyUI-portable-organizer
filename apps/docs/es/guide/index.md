# Qué es esto

ComfyUI Portable Organizer es una aplicación de escritorio para Windows que
gestiona varias instalaciones portables de ComfyUI: un registro de tus
instalaciones, un lanzador y el propio ComfyUI dibujado como pestaña dentro
de la ventana de la aplicación.

Es una envoltura alrededor de carpetas que ya tienes. Nunca parchea
ComfyUI, nunca edita tus archivos `.bat` y nunca toca `custom_nodes`.

## El vocabulario

**Instalación** — una carpeta portable de ComfyUI, de las que salen al
descomprimir `ComfyUI_windows_portable_nvidia.7z`. Contiene
`python_embeded\` y `ComfyUI\`, y se basta a sí misma. La aplicación llama
instalaciones a estas carpetas; el registro es la lista de ellas.

**Perfil de arranque** — una forma de iniciar una instalación. Cada archivo
`.bat` que viene en el paquete portable (`run_nvidia_gpu`, `run_cpu`,
`advanced\run_nvidia_gpu_disable_api_nodes`…) se convierte en un perfil.
Puedes añadir los tuyos encima; los `.bat` en sí quedan intactos.

**Carpeta de modelos compartida** — una carpeta fuera de las instalaciones
donde viven los archivos pesados. Las instalaciones apuntan a ella
mediante el mecanismo propio de ComfyUI: `extra_model_paths.yaml`.

**Biblioteca de flujos** — una carpeta fuera de las instalaciones donde
viven los grafos que te importan, con etiquetas, notas y favoritos.

## La versión corta

1. [Instala la aplicación](/es/guide/install-app) y pasa el aviso de
   SmartScreen.
2. [Añade una instalación](/es/guide/add-build) — señala una carpeta que ya
   tengas o [descomprime una desde un archivo](/es/guide/install-from-archive).
3. Pulsa **Iniciar**. La aplicación elige un puerto libre, añade
   `--disable-auto-launch` para que no se abra ningún navegador y
   retransmite el registro de arranque hasta que el servidor responde.
   Entonces ComfyUI aparece en una pestaña.
4. Opcional: conecta la instalación a una
   [carpeta de modelos compartida](/es/guide/shared-models) y a una
   [biblioteca de flujos](/es/guide/workflows).

## Lo que no va a hacer

- Editar grafos, dibujar vistas previas del lienzo o versionar tus flujos.
- Instalar o actualizar nodos personalizados: eso es trabajo de
  ComfyUI-Manager. La aplicación solo te dice qué nodos necesita un flujo
  y no tiene una instalación.
- Cambiar el tema o el idioma del propio ComfyUI.
- Compartir `custom_nodes` entre instalaciones, jamás. Los nodos
  personalizados en conflicto son la razón por la que existen instalaciones
  separadas.
- Borrar tus modelos o tus flujos. La lista exacta de qué se elimina y qué
  no está en [Desinstalar](/es/guide/uninstall).
