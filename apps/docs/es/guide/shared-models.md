# Modelos compartidos

Cada instalación portable lleva su propio `ComfyUI\models\`. Un checkpoint
ocupa entre 2 y 20 GB, así que tres o cuatro instalaciones significan
cientos de gigabytes de los mismos archivos. ComfyUI tiene un mecanismo
justo para esto — `extra_model_paths.yaml` — y el trabajo de la aplicación
es generar uno correcto y entregárselo a la instalación.

## Cómo organizar la carpeta

Elige una carpeta fuera de todas las instalaciones, en una unidad con
espacio:

```
D:\AI\_shared_models\
├─ checkpoints\
├─ loras\
├─ vae\
├─ controlnet\
├─ upscale_models\
├─ embeddings\
├─ text_encoders\        (o clip\)
└─ diffusion_models\     (o unet\)
```

Después ve a **Ajustes → Modelos compartidos**, elige la carpeta y la
aplicación la analiza.

**Las secciones se construyen a partir de las subcarpetas que existen de
verdad**, no de una lista fija de categorías. Es a propósito: ComfyUI añade
tipos de modelo entre versiones, y una lista congelada en nuestro código se
quedaría obsoleta. Añade una subcarpeta, vuelve a analizar y aparecerá en
la configuración.

Dos nombres heredados se mapean por ti: `unet\` se ofrece bajo
`diffusion_models` y `clip\` bajo `text_encoders`, que es lo que hace el
propio ComfyUI internamente. Pueden existir a la vez la carpeta heredada y
la moderna, y las dos acaban en la misma clave.

Las subcarpetas que la aplicación no reconoce se listan aparte en lugar de
descartarse en silencio: verás qué se quedó fuera.

`custom_nodes` se excluye sin condiciones, aunque la clave sea válida y
aparezca en el archivo de ejemplo del propio ComfyUI. Compartir nodos
personalizados echa por tierra la razón por la que tus instalaciones están
separadas: entran en conflicto entre sí, y una carpeta compartida las
rompería todas de golpe.

## Tus modelos locales no se sustituyen

Las rutas se **suman**. ComfyUI añade las rutas extra a las que la
instalación ya tiene, así que los modelos que hay dentro de ella siguen
funcionando exactamente igual que antes. Conectar una carpeta no mueve nada
a ninguna parte.

El interruptor **«Descargar aquí los modelos nuevos»** controla una sola
cosa: si la carpeta compartida va primera en la lista, que es donde el
ComfyUI Manager deja los archivos recién descargados. No afecta a lo que se
encuentra.

## Conectar una instalación

En la pestaña **Modelos** de la instalación hay un interruptor. Actívalo y
la instalación usará la carpeta compartida a partir del siguiente arranque.
Los argumentos de un proceso en marcha no se pueden cambiar, así que la
aplicación dice claramente que hace falta reiniciar en vez de fingir que el
interruptor ya ha surtido efecto.

## Dos formas de aplicarlo

**Modo opción — el predeterminado.** El YAML generado vive en la carpeta de
datos de la propia aplicación, y `--extra-model-paths-config <ruta>` se
añade a la línea de comandos al arrancar. Dentro de tu instalación no se
escribe absolutamente nada. Desactivarlo es desmarcar una casilla; una sola
configuración sirve a todas las instalaciones y se edita en un solo sitio.

**Archivo dentro de la instalación.** La aplicación escribe
`<instalación>\ComfyUI\extra_model_paths.yaml`. Elige esto si a veces
arrancas la instalación con su `.bat`, fuera de la aplicación: el archivo
lo recoge ComfyUI por su cuenta.

La política ante colisiones en ese modo es estricta:

- no hay archivo → se escribe;
- está el nuestro (reconocido por una línea marcadora en la cabecera) → se
  reescribe;
- **está el de otro → no se toca.** Recibes una pantalla de comparación, el
  original se respalda como `extra_model_paths.yaml.bak-<marca de tiempo>`
  y el archivo se sustituye solo después de que confirmes. La alternativa
  que se ofrece en esa misma pantalla es dejarlo en paz y usar el modo
  opción.

Desconectar en este modo elimina nuestro archivo y restaura el respaldo si
lo había.

## Si la carpeta no está

Una unidad externa que no está conectada se detecta **antes** de que la
instalación arranque, con la oferta de arrancar sin modelos compartidos.
Enterarse por un «model not found» a mitad de una generación no es una
opción.

## Mover modelos a la carpeta compartida

La pestaña **Modelos** de la instalación lista lo que hay dentro de ella y
lo que ya existe en la carpeta compartida con el mismo nombre y tamaño.

- **Mover a la carpeta compartida** copia el archivo, vuelve a leer la
  copia, la compara y solo entonces elimina el original. Hasta que la copia
  esté verificada, el original se queda donde está.
- **Eliminar duplicados** borra de la instalación los archivos que ya están
  en la carpeta compartida — de nuevo, tras comprobar que el gemelo está
  realmente ahí.

Ambas se niegan a funcionar sobre una instalación en marcha: ComfyUI
mantiene esos archivos abiertos y resolvió sus rutas al arrancar.

Nunca se borra nada sin que lo pidas, y se te dice exactamente qué se va a
ir antes de que se vaya.

## Comprobar que ha funcionado

La comprobación más directa no pasa por la interfaz. Con la instalación en
marcha, abre
`http://127.0.0.1:<puerto>/internal/folder_paths` — ahí están todas las
categorías con las rutas que hay detrás, carpeta compartida incluida, y con
«descargar aquí los modelos nuevos» activado la ruta compartida va primera
en su lista.

El registro de arranque enseña lo mismo sobre la marcha: ComfyUI anota
`Adding extra search path checkpoints D:\…` por cada ruta que asume.
