# Biblioteca de flujos

Los flujos de trabajo viven dentro de cada instalación, en
`user\default\workflows\`, y no viajan entre instalaciones. La biblioteca
es una carpeta fuera de todas ellas donde se guardan los grafos que de
verdad usas, con etiquetas, notas y favoritos.

La carpeta se indica en **Ajustes → Biblioteca de flujos**. La aplicación
sugiere una junto a tu carpeta de modelos compartida, pero no hay ningún
vínculo rígido entre ambas: la biblioteca funciona sin modelos compartidos
y al revés.

## La biblioteca es una carpeta, no una base de datos

```
<biblioteca>\
├─ _library.json      manifiesto: etiquetas, notas, favoritos
├─ sdxl\
│  └─ base.json
└─ upscale.json
```

El manifiesto vive **dentro de la biblioteca**, no en la carpeta de datos
de la aplicación, así que la biblioteca se basta a sí misma: sobrevive a
una reinstalación de la aplicación y se muda a otra máquina como una sola
carpeta.

Editar esa carpeta en el Explorador no rompe nada. Un archivo sin entrada
en el manifiesto es válido y se muestra tal cual; una entrada cuyo archivo
ha desaparecido se marca como perdida, con la oferta de eliminar el
registro. Los archivos son la verdad; el manifiesto solo los enriquece.

## Cómo llenarla

- **Añadir archivo** y arrastrar y soltar `.json` sobre la pantalla de la
  biblioteca.
- **Suelta un PNG** generado por ComfyUI: el grafo viaja dentro de la
  imagen, en su fragmento de texto `workflow`, y la aplicación lo extrae.
  Volver a una buena generación suele empezar por la imagen.
- **Pegar como texto.** Los grafos se comparten en chats y foros como texto
  más a menudo que como archivos. El formulario de pegado lee el
  portapapeles por su cuenta, analiza mientras escribes y muestra el número
  de nodos antes de guardar: la única forma de ver que has pegado lo
  correcto sin leer dos mil líneas. El nombre se pide de inmediato y se
  valida, porque se convierte en una ruta.
- **Recoger de una instalación** — desde la pestaña **Flujos** de la
  instalación.

## Recoger mueve, no copia

Llevar un flujo de una instalación a la biblioteca lo quita de la
instalación. El orden es el mismo que con los modelos, y por el mismo
motivo: la copia se escribe, se vuelve a leer y se compara, y solo entonces
se elimina el original — a través de la propia API de ComfyUI si la
instalación está en marcha, para que sepa que su carpeta ha cambiado.

**No hay sobrescritura.** Un nombre ya ocupado se resuelve comparando
contenidos, no pidiéndote que sustituyas algo:

- idénticos → no hay nada que recoger, el botón está desactivado;
- distintos → son dos trabajos diferentes, y el de la instalación se recoge
  con un nombre libre (`base.json` → `base (2).json`) tras preguntar.

Sustituir machacaría un trabajo con otro y te dejaría sin ninguno de los
dos.

La comparación mira primero los bytes y después el JSON analizado. El
segundo paso no es opcional: ComfyUI reescribe el archivo en cada guardado,
así que sin él casi todos los flujos ya recogidos se declararían distintos.

## Añadir un flujo a una instalación

- **Instalación en marcha** → se sube por su API con `overwrite=false`. Un
  409 de ComfyUI se convierte en una pregunta —sobrescribir o guardar con
  otro nombre—, nunca en una sustitución silenciosa.
- **Instalación detenida** → se copia en `user\default\workflows\`. Si el
  perfil tiene `--user-directory`, esa ruta se resuelve a partir de él en
  lugar de darse por supuesta.

La lista dentro de un ComfyUI en marcha se actualiza cuando lo hace su
página.

Las operaciones en lote funcionan en ambos sentidos: varios flujos a una
instalación, un flujo a varias instalaciones.

## El aviso de nodos que faltan

Por esto la función vive aquí y no en el Explorador. La aplicación saca los
tipos de nodo del JSON del flujo y los compara con lo que una instalación
tiene de verdad:

- **Instalación en marcha** — preguntada directamente, ahora mismo.
- **Instalación detenida** — comparada contra una instantánea tomada en su
  último arranque correcto, y así se indica.
- **Nunca arrancada mientras mirábamos** — *desconocido*, y como
  desconocido se muestra. Una marca verde sin nada detrás es peor que
  ninguna marca.

Es un aviso, no un bloqueo. Instalar los nodos que faltan es trabajo de
ComfyUI-Manager; la aplicación solo te dice qué buscar.

## Lo que deliberadamente no hace

Ni edición de grafos, ni vistas previas del lienzo, ni versionado, ni
instalación de los nodos que faltan.

Dos atajos descartados, por si ibas a construirlos tú: compartir todo el
directorio `user\` mediante `--user-directory` comparte también los ajustes
del frontend y el estado de las extensiones, que entran en conflicto entre
instalaciones con distintos conjuntos de nodos — justo el problema que las
instalaciones separadas existen para evitar. Un junction sobre la carpeta
`workflows` significa que editar un flujo lo cambia en silencio en todas
partes, y que borrar la carpeta una vez la borra para todos.
