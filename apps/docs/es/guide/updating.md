# Actualizar la aplicación

La aplicación busca una versión nueva al arrancar y muestra lo que ha
encontrado en **Acerca de**, junto con las notas de esa versión.

## La comprobación es lo único que sale de tu ordenador

Envía a GitHub el número de la versión actual y pregunta si existe una
publicación más nueva. Nada más sobre ti, sobre tus instalaciones o sobre
tus archivos se envía jamás a ninguna parte.

Se puede desactivar: **Acerca de → Buscar actualizaciones al arrancar**.
Desactivada, no se ejecuta en absoluto: no hay un segundo calendario
escondido. Aun así puedes pulsar **Buscar ahora** cuando quieras.

Sin red no pasa nada y no se informa de nada. Un error de red que no has
pedido parece una aplicación rota, y la aplicación no está rota; lo que
falta es tu conexión.

## Nada se instala sin ti

La actualización se descarga y se instala solo después de que pulses el
botón. Su firma se verifica antes de instalar nada, y una actualización que
no verifica no se instala — se te dice por qué. Esa firma es un par de
claves `minisign` cuya mitad pública está compilada dentro de la
aplicación; no tiene nada que ver con
[el aviso de SmartScreen](/es/guide/smartscreen), que es un mecanismo
completamente distinto.

## Las instalaciones en marcha son decisión tuya

Instalar una actualización cierra la aplicación —así funcionan los
instaladores de Windows— y todos los servidores de ComfyUI en marcha se van
con ella, porque son sus procesos hijos.

Por eso, si hay algo en marcha, la instalación no empieza. Recibes la lista
de instalaciones activas y dos opciones:

- **Detenerlas e instalar** — los servidores se detienen como es debido y
  luego la actualización sigue adelante;
- **Posponer hasta el próximo arranque** — ahora no pasa nada, y la misma
  actualización se ofrece de nuevo la próxima vez que abras la aplicación.

Tu cola de generación vale más que ir una versión por detrás durante una
hora.

## Tus datos sobreviven a la actualización

Instalar una versión nueva sobre una antigua se ejecuta en modo
actualización, que se salta por completo la limpieza de datos. Los ajustes,
el registro de instalaciones, las conexiones a carpetas compartidas y la
biblioteca de flujos siguen donde los dejaste.

Las actualizaciones manuales funcionan igual: descarga el instalador nuevo
y ejecútalo sobre la versión antigua.
