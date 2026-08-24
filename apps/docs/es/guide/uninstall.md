# Desinstalar

Se desinstala desde **Configuración → Aplicaciones → Aplicaciones
instaladas**, por la vía normal de Windows. No hacen falta permisos de
administrador, porque la aplicación se instaló en tu perfil de usuario.

El desinstalador pregunta si borrar también los datos de la aplicación.

## Qué desaparece

| Casilla «borrar los datos de la aplicación» | Se elimina |
| --- | --- |
| La aplicación en sí | siempre |
| `%APPDATA%\io.github.quo0.comfyui-organizer` — ajustes, registro de instalaciones, ruta de tu biblioteca | solo si está marcada |
| `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer` — caché y datos de WebView2 | solo si está marcada |

Dejar la casilla sin marcar conserva tu registro y tus ajustes, y una
reinstalación posterior los recoge tal como estaban.

## Qué se queda, siempre

- **Tus modelos.** Tanto la carpeta compartida como los que hay dentro de
  cada instalación.
- **Tu biblioteca de flujos**, incluido su `_library.json`.
- **Las propias instalaciones de ComfyUI.** El registro solo recordaba
  dónde están.
- **Los dos tipos de archivo que la aplicación escribió dentro de tus
  instalaciones**, si se lo pediste: `ComfyUI\extra_model_paths.yaml` en
  modo «archivo dentro de la instalación» y las copias de flujos que
  añadiste a una instalación. Están dentro de la instalación de otro, y
  sacarlas de ahí no nos corresponde.

Todo esto figura en la propia aplicación, en la pantalla **Acerca de**,
antes de que desinstales nada — con un botón junto a cada ruta que la abre
en el Explorador.

## Nunca se borra nada en silencio

La aplicación borra contenido tuyo exactamente en tres situaciones, y cada
una empieza con un clic tuyo, después de haberte dicho qué se va a ir:

1. **Mover modelos a la carpeta compartida** — el original se elimina solo
   después de que la copia se haya escrito, releído y comparado.
2. **Eliminar modelos duplicados** — solo archivos que se ha comprobado que
   ya están en la carpeta compartida.
3. **Recoger un flujo en la biblioteca** — mismo orden que con los modelos:
   copiar, verificar y luego eliminar.

Fuera de esas tres, nada tuyo se toca.

## Limpiar a mano después

Si dejaste la casilla sin marcar y luego cambias de idea, borra estas dos
carpetas:

```
%APPDATA%\io.github.quo0.comfyui-organizer
%LOCALAPPDATA%\io.github.quo0.comfyui-organizer
```

No hay nada más. La aplicación no escribe nada en `Documentos`, nada en
`%PROGRAMDATA%` y nada en el registro de Windows más allá de lo que el
instalador pone ahí para la entrada de desinstalación.
