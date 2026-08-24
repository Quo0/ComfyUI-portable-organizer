# Instalar la aplicación

## Requisitos

- Windows 10 u 11, 64 bits.
- El entorno de ejecución WebView2. Viene con Windows 11 y con cualquier
  Windows 10 reciente; si falta, el instalador lo trae.
- Al menos una instalación portable de ComfyUI — o un archivo que
  descomprimir, cosa que la aplicación puede hacer por ti.

No hacen falta permisos de administrador.

## La instalación

1. [Descarga el instalador](/es/download).
2. Windows muestra **«Windows protegió su PC»**. Es lo esperado y no es un
   aviso de virus: mira
   [la página sobre SmartScreen](/es/guide/smartscreen) para saber qué
   pulsar y cómo verificar el archivo.
3. La aplicación se instala en tu perfil de usuario
   (`%LOCALAPPDATA%\Programs\ComfyUI Portable Organizer`) y arranca.

## Dónde guarda la aplicación sus propios datos

| Qué | Dónde |
| --- | --- |
| Ajustes, registro de instalaciones, ruta de tu biblioteca | `%APPDATA%\io.github.quo0.comfyui-organizer` |
| Caché y datos de navegación de WebView2 | `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer` |

Ambas carpetas aparecen en la aplicación, en **Acerca de**, cada una con un
botón que la abre en el Explorador. No se escribe nada en `Documentos`, ni
en `%PROGRAMDATA%`, ni junto al ejecutable.

Tus modelos, tus flujos y tus instalaciones de ComfyUI viven donde tú los
pusiste, y la aplicación no los lleva ni los trae. Véase
[Desinstalar](/es/guide/uninstall).

## El primer arranque

La aplicación se abre con el registro vacío y una sola cosa que hacer:
añadir una instalación. El idioma sigue al de Windows en el primer arranque
— inglés, ruso, chino simplificado y español vienen incluidos — y el tema
sigue al del sistema hasta que elijas uno en
**Ajustes → Apariencia**.
