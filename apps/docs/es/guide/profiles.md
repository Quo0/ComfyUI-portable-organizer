# Perfiles de arranque

Un perfil es una forma de iniciar una instalación. Los archivos `.bat` que
vienen en el paquete portable se convierten en perfiles automáticamente,
incluidos los de `advanced\`.

## Los `.bat` se leen, no se ejecutan

Esos archivos son de una línea:

```bat
.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build --fast fp16_accumulation
```

La aplicación analiza la línea y arranca `python.exe` directamente. Con
ello desaparecen dos cosas: el `pause` del final del archivo, que se
quedaría esperando a que pulses Intro, y el `cmd.exe` de más en el árbol de
procesos, que hace menos fiable detener el servidor.

Las rutas relativas se resuelven desde la carpeta del propio archivo
`.bat` — es lo que pasa cuando haces doble clic en él, y los
`advanced\*.bat` usan rutas con `..\` que dependen de ello.

**Tus archivos `.bat` no se modifican nunca.** Se releen en cada arranque,
que es también por lo que los cambios que hagas en ellos se ven al momento.

Si un archivo no se puede analizar, el perfil recurre a ejecutarlo mediante
`cmd /c` y así queda marcado en la interfaz. Detener un proceso de ese tipo
es menos fiable, y conviene saberlo antes de depender de él.

## Qué añade la aplicación a la línea de comandos

Tres opciones, y el comando resultante exacto se ve en el editor de
argumentos antes de arrancar:

| Opción | Por qué |
| --- | --- |
| `--port <n>` | El puerto lo asigna la aplicación. Cualquier `--port` que ya hubiera en el perfil se elimina junto con su valor. |
| `--disable-auto-launch` | Para que no se abra ninguna pestaña del navegador. ComfyUI aplica esta opción después de `--windows-standalone-build`, así que siempre gana. |
| `--extra-model-paths-config <ruta>` | Solo cuando la instalación está conectada a una [carpeta de modelos compartida](/es/guide/shared-models) en modo opción. |

Un `--extra-model-paths-config` que ya estuviera en el `.bat` se deja en
paz: la opción se acumula, y la nuestra se añade junto a la tuya en lugar
de sustituirla.

## Tus propios perfiles

**Ajustes de una instalación → Parámetros** te permite construir un perfil
propio encima de uno analizado: cambia los argumentos y guárdalo con el
nombre que quieras. Los originales se quedan como están, porque son
archivos dentro de la instalación de otro.

El editor muestra la línea de comandos final — la que de verdad se le va a
entregar a Windows, ya con nuestras opciones aplicadas. Discutir con una
línea de comandos invisible no tiene ninguna gracia.

## Qué perfil arranca

El botón **Iniciar** es un botón partido: la mitad principal arranca el
último perfil que usaste para esa instalación, la flecha abre la lista. Los
perfiles que no levantan un servidor — los scripts de actualización que
vienen en el paquete — quedan fuera de esa lista, porque no son arranques.

## El registro de arranque

Todo lo que el proceso escribe en stdout y stderr se retransmite a la
aplicación, en vivo, desde la primera línea. Las barras de progreso de
`tqdm` sobrescriben su línea en lugar de inundar el búfer, igual que en un
terminal. Se conservan unas 5000 líneas.

Mientras una instalación arranca, la aplicación consulta
`GET /system_stats` hasta que el servidor responde, y entonces cambia el
registro por la pestaña de ComfyUI. Un arranque en frío de una instalación
grande tarda minutos; el tiempo de espera son cinco.
