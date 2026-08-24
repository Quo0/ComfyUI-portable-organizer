# Perfiles de arranque

Un perfil es lo que significa un archivo `.bat`, en una forma que la
aplicación pueda arrancar directamente. Los perfiles se vuelven a derivar
del disco en cada arranque: los `.bat` son la fuente de verdad, y nunca se
modifican.

## La forma

| Campo | Significado |
| --- | --- |
| `id` | Ruta del `.bat` relativa a la raíz de la instalación. También es su identidad: `run_nvidia_gpu.bat`, `advanced\run_nvidia_gpu_disable_api_nodes.bat`. |
| `name` | Nombre del archivo sin la extensión. Nunca se traduce. |
| `advanced` | Vino de la subcarpeta `advanced\`. |
| `pythonPath` | Ruta absoluta al intérprete que invoca la línea. |
| `args` | Los argumentos, separados en tokens, respetando las comillas. |
| `cwd` | La carpeta del propio archivo `.bat`. |
| `env` | Variables de las líneas `set CLAVE=VALOR` que hay encima del comando. |
| `fallback` | El análisis falló; el archivo se ejecutará mediante `cmd /c`. |

Tus perfiles propios reutilizan todo de un perfil base y sustituyen solo
`args` — véase [`instances.json`](/es/reference/instances-json).

## Cómo se lee el archivo

`echo`, `pause`, `rem` y las líneas en blanco se saltan. Las líneas
`set CLAVE=VALOR` se convierten en variables de entorno. La primera línea
de comando real se separa en tokens respetando las comillas, el ejecutable
pasa a ser `pythonPath` y el resto pasa a ser `args`.

Las rutas relativas — incluido el `..\` de `advanced\*.bat` — se resuelven
contra la carpeta del archivo `.bat`, porque es lo que pasa cuando haces
doble clic en él. Esa carpeta pasa a ser también el directorio de trabajo.

Los perfiles que no levantan un servidor, como los scripts de actualización
que vienen dentro del paquete, se quedan fuera de la lista de arranque.

## `fallback: true`

Cuando una línea no se puede analizar, el perfil no se descarta: se marca
como de reserva y se arranca mediante `cmd /c`. La interfaz lo dice, porque
esa vía mete un `cmd.exe` de más entre la aplicación y Python y hace menos
fiable detener el proceso.

## Rutas derivadas de los argumentos

Tres carpetas se resuelven a partir de los argumentos en lugar de darse por
supuestas, siguiendo la misma precedencia que usa el propio ComfyUI:

| Carpeta | Opciones consultadas, en orden |
| --- | --- |
| Modelos | `--base-directory`, si no `<instalación>\ComfyUI\models` |
| Datos de usuario (aquí viven los flujos) | `--user-directory`, luego `--base-directory`, si no `<instalación>\ComfyUI\user` |
| Salida | `--output-directory`, luego `--base-directory`, si no `<instalación>\ComfyUI\output` |

Por esto «abrir la carpeta de salida» y «añadir un flujo a una instalación
detenida» aciertan el sitio incluso cuando tu perfil mueve esas carpetas a
otra parte.
