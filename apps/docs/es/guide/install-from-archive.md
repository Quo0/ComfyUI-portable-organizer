# Instalar desde un archivo

El `ComfyUI_windows_portable_nvidia.7z` lo descargas tú, del proyecto
ComfyUI. La aplicación lo descomprime — en una carpeta o en varias a la
vez —, registra el resultado y puede conectarlo a tus carpetas compartidas
en la misma pasada.

Esto vive en **Añadir → Instalar desde un archivo**, y es una sección
permanente, no un paso del primer arranque: así es también como se despliega
una versión nueva de ComfyUI junto a las anteriores.

## Los pasos

1. **Archivo.** Elige el `.7z` o reutiliza uno del historial. Antes de
   usarlo, la aplicación comprueba que el archivo sigue ahí y que su tamaño
   y su fecha de modificación no han cambiado: los archivos se borran y se
   sustituyen.
2. **Lectura.** De la cabecera se sacan el número de entradas, el tamaño
   total sin comprimir y la única carpeta raíz que hay dentro. En un
   archivo de 2 GB con 56 000 entradas esto tarda alrededor de un segundo,
   y la pantalla lo dice.
3. **Destinos.** Uno o varios. Cada uno lleva su ruta, su nombre, su
   descripción y su color de acento. La aplicación comprueba que la carpeta
   está vacía o no existe, que hay espacio suficiente y que la ruta no es
   peligrosamente larga.
4. **Recursos compartidos.** Interruptores opcionales (pero muy
   recomendables) para la
   [carpeta de modelos compartida](/es/guide/shared-models) y la
   [biblioteca de flujos](/es/guide/workflows).
5. **Descompresión** y luego **Listo**, con las instalaciones nuevas
   preparadas para arrancar.

## Elige una ruta de destino corta

El archivo más profundo dentro del paquete queda unos 206 caracteres por
debajo de la raíz. El límite clásico de rutas de Windows es 260, así que un
destino como `D:\AI\comfy-sdxl` va holgado y
`C:\Users\tú\Documents\cosas de IA\instalaciones ComfyUI\experimentos\sdxl`
no.

La aplicación avisa cuando la ruta es larga. Ella descomprime usando rutas
verbatim `\\?\`, así que aguanta más que el Explorador, pero las
herramientas que uses después — incluidos los instaladores de nodos
personalizados de ComfyUI — puede que no.

## Varios destinos a la vez

La descompresión ocurre **una sola vez**. El segundo destino y los
siguientes se copian del primero, porque descomprimir depende de la CPU y
copiar un árbol ya listo depende del disco. Con dos o tres destinos esto
ahorra minutos.

## Interrumpir es seguro

Los archivos van a una carpeta temporal `<destino>.cpo-partial` y solo se
mueven a su sitio cuando la descompresión termina. Eso no es decoración: un
árbol descomprimido a medias ya contiene `python_embeded\python.exe` y
`ComfyUI\main.py`, así que pasaría la propia comprobación de validez de la
aplicación y parecería una instalación funcional.

Cancelar — o un fallo — elimina la carpeta temporal. Limpiar decenas de
miles de archivos lleva su tiempo y se informa como una fase propia, para
que una instalación cancelada no parezca colgada.

## Espacio libre

Se comprueba antes de empezar nada, a partir de la cabecera del archivo,
con margen y multiplicado por el número de destinos que caen en el mismo
volumen. Quedarse sin espacio a diecinueve gigabytes de una descompresión
de veinte es el peor momento posible para enterarse.
