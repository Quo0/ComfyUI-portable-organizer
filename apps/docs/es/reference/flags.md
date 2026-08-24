# Opciones que añadimos

La aplicación arranca `python.exe` directamente, con los argumentos
extraídos del perfil más un máximo de tres propios. La línea de comandos
resultante se ve en el editor de argumentos antes de arrancar nada.

## `--port <n>`

El puerto lo asigna la aplicación: el puerto preferido de la instalación si
está libre, y el siguiente libre si no.

Cualquier `--port` ya presente en el perfil se elimina **junto con su
valor**, en ambas grafías (`--port 8188` y `--port=8188`). Dos
instalaciones con el mismo puerto fijo significarían que la segunda falla
al arrancar por un motivo que no parece tener relación con nada de lo que
hiciste.

## `--disable-auto-launch`

Para que arrancar una instalación no abra una pestaña del navegador.

Esto funciona sin parchear nada, por el orden dentro del propio tratamiento
de argumentos de ComfyUI (`comfy/cli_args.py`):

```python
if args.windows_standalone_build:
    args.auto_launch = True
if args.disable_auto_launch:
    args.auto_launch = False
```

La opción que desactiva se aplica en segundo lugar y siempre gana, así que
basta con añadirla a la línea de comandos.

## `--extra-model-paths-config <ruta>`

Se añade solo cuando la instalación está conectada a una carpeta de modelos
compartida en **modo opción**. La ruta apunta a la configuración que hay en
la carpeta de datos de la propia aplicación, que se regenera cada vez que
cambian los ajustes.

Un `--extra-model-paths-config` ya presente en el perfil **no** se toca. La
opción se acumula (`nargs='+'`, `action='append'`), y ComfyUI carga primero
`ComfyUI/extra_model_paths.yaml` y después los archivos de la opción, así
que el nuestro se suma al tuyo en lugar de sustituirlo.

En el modo «archivo dentro de la instalación» no se añade ninguna opción:
el archivo está en la carpeta de la instalación y ComfyUI lo recoge solo.

## Lo que no añadimos nunca

- `--enable-cors-header`. Permitiría incrustar ComfyUI en un `<iframe>` —
  y desactiva por completo la protección entre sitios de ComfyUI, lo que
  significa que cualquier web abierta en tu navegador podría hablar con tu
  servidor local. En su lugar, la aplicación usa una ventana hija nativa,
  que el propio middleware de ComfyUI acepta como una navegación normal de
  nivel superior.
- `--listen`. El servidor se queda en `127.0.0.1`.
- Cualquier cosa que cambie las rutas de modelos más allá de la
  configuración de arriba.
