# Limitaciones conocidas

Cosas que te sorprenderán si nadie las dice en voz alta.

## Dos instalaciones comparten una GPU

Nada te impide ejecutar dos instalaciones a la vez, y la aplicación
pregunta antes de dejarte. Pero la memoria de vídeo no se reparte: la
segunda instalación carga sus modelos en la misma GPU y falla dentro de una
cola de generación, con un error que no dice nada de la causa real. Dos
GPU, o un perfil de CPU, son los casos en los que arrancar de todas formas
tiene sentido.

## ComfyUI-Manager reinicia el servidor a nuestras espaldas

Tras instalar nodos personalizados, ComfyUI-Manager mata el servidor y
arranca un proceso nuevo. Nuestro control sobre él se pierde, así que la
instalación se muestra como **desacoplada**: viva, pero ya no es nuestra
para detenerla. La aplicación ofrece reconectar — encuentra el proceso
dueño del puerto y lo recupera. Véase
[Puertos y conflictos](/es/guide/ports#cuando-comfyui-se-reinicia-solo).

## Cerrar la aplicación detiene todos los servidores

Los procesos de ComfyUI son hijos de la aplicación y se mantienen en un Job
Object que se los lleva consigo. Es deliberado —un `python.exe` huérfano
reteniendo tu memoria de vídeo es peor—, pero implica que el botón de
cerrar la ventana hace una pregunta cuando hay algo en marcha, y lo mismo
al instalar una actualización.

Si quieres que los servidores sigan trabajando, minimiza a la bandeja.

## La pestaña de ComfyUI es una ventana nativa

No es un iframe. Eso tiene consecuencias visibles: nada nuestro puede
dibujarse encima de ese rectángulo, y por eso los ajustes, el editor de
argumentos y todas las confirmaciones son pantallas aparte en lugar de
diálogos, y por eso los mensajes aparecen como una franja encima de la
pestaña y no como una notificación sobre ella.

Las descargas y el «guardar imagen» dentro de la pestaña los gestiona
WebView2, no nosotros: se comportan como lo harían en Edge.

## Rutas largas

El archivo más profundo de una instalación portable queda unos 206
caracteres por debajo de su raíz, frente al límite clásico de Windows de
260. La aplicación descomprime usando rutas verbatim y aguanta más que el
Explorador, pero las herramientas que ejecutes después — instaladores de
nodos personalizados incluidos — puede que no. Mantén los destinos cortos:
`D:\AI\comfy-sdxl`, no una carpeta cinco niveles dentro de tu perfil.

## Los modelos se comparan por nombre y tamaño, no por contenido

El informe de duplicados y las marcas de «ya está en la carpeta
compartida» comparan nombres y tamaños de archivo. Calcular el hash de un
checkpoint de 20 GB para estar seguros llevaría minutos por archivo, y el
informe está pensado para mirarlo, no para esperarlo.

El informe de duplicados no toca ningún archivo: solo informa. Borrar es
una operación aparte y explícita, en la pestaña Modelos de la instalación.

## Solo Windows

macOS y Linux no se compilan ni se prueban. Las instalaciones portables de
ComfyUI son un arreglo de Windows para empezar; en otros sistemas la gente
usa un entorno virtual, y el problema que esta aplicación resuelve no
existe con la misma forma.

## Sin firma de código

El instalador no está firmado, así que SmartScreen avisa sobre él. Véase
[la página sobre SmartScreen](/es/guide/smartscreen) — incluido cómo
verificar el archivo en vez de confiar en él.
