# Puertos y conflictos

## El puerto lo elige la aplicación

Cada instalación tiene un **puerto preferido** — 8188 por defecto, el mismo
que el de ComfyUI. Al arrancar una instalación, la aplicación toma ese
puerto si está libre y el siguiente libre si no lo está, y luego lo pasa
como `--port`.

Un `--port` escrito en el archivo `.bat` se elimina de la línea de comandos
junto con su valor. Dos instalaciones arrancadas con el mismo puerto fijo
significarían que la segunda falla al arrancar con un mensaje que no tiene
nada que ver con lo que hiciste.

## Dos instalaciones a la vez

Arrancar una segunda instalación mientras otra está en marcha está
permitido, pero la aplicación pregunta antes. El motivo no es el puerto
—eso lo resuelve— sino la memoria de vídeo: la segunda instalación cargará
sus modelos en la misma GPU y fallará en algún punto de la cola de
generación, con un error que no dice nada de la causa real.

La decisión es tuya: **detener la otra** o **arrancar de todas formas**. En
una máquina con dos GPU, o con un perfil de CPU, arrancar de todas formas
es exactamente lo correcto.

## Detener

**Detener** termina el árbol de procesos y espera a que el puerto se libere
antes de dar la instalación por detenida. Esa espera importa: sin ella, un
reinicio competiría con el sistema operativo por el puerto.

Todos los procesos de ComfyUI arrancados por la aplicación viven en un Job
Object de Windows con `KILL_ON_JOB_CLOSE`. Si se mata la propia aplicación
—desde el Administrador de tareas o por un instalador durante una
actualización—, los servidores se van con ella. Es deliberado: un
`python.exe` huérfano reteniendo veinte gigabytes de memoria de vídeo es
peor que un servidor detenido.

Es también por lo que la aplicación pregunta qué hacer con las
instalaciones en marcha cuando cierras la ventana o instalas una
actualización.

## Cuando ComfyUI se reinicia solo

ComfyUI-Manager reinicia el servidor tras instalar nodos personalizados:
mata el proceso y arranca uno nuevo. La aplicación pierde el control sobre
él — el PID que conocía ya no existe, y el puerto sigue respondiendo.

Ese estado se muestra como **desacoplado**: el servidor está vivo pero no
es nuestro, y fingir lo contrario sería mentir. La aplicación consulta el
puerto durante 15 segundos tras una salida así y luego ofrece
**reconectar**: encuentra el proceso dueño del puerto, anota su PID y todo
vuelve a funcionar, detenerlo incluido.

La pestaña no se cierra mientras la instalación está desacoplada. El
servidor que hay detrás sigue vivo, y quitarte una interfaz que funciona
sería una grosería.

## Caídas

Si el proceso termina con un código distinto de cero, la instalación se
marca como **caída**, se muestra el código de salida y aparece un
distintivo junto a ella en el raíl — aunque estés en otra pantalla cuando
ocurra. El registro se queda donde está, para que puedas leer el final.

No se lanza ninguna notificación por una caída mientras su pestaña está
abierta: el estado en el raíl y el registro lo dicen mejor, y una
notificación se la tragaría de todos modos la ventana nativa de ComfyUI.
