# Referencia

Qué escribe la aplicación, dónde lo escribe y qué le entrega a ComfyUI.
Esta sección existe para que puedas comprobar la aplicación en lugar de
confiar en ella.

- [Opciones que añadimos](/es/reference/flags) — las tres opciones de línea
  de comandos y por qué está cada una.
- [instances.json](/es/reference/instances-json) — el formato del registro
  de instalaciones.
- [Perfiles de arranque](/es/reference/launch-profile) — cómo un archivo
  `.bat` se convierte en un perfil.
- [El YAML generado](/es/reference/extra-model-paths) — la configuración de
  modelos compartidos que producimos.

## Dónde está cada cosa

| Ruta | Qué |
| --- | --- |
| `%APPDATA%\io.github.quo0.comfyui-organizer\settings.json` | Ajustes: tema, idioma, modelos compartidos, ruta de la biblioteca |
| `%APPDATA%\io.github.quo0.comfyui-organizer\instances.json` | El registro de instalaciones |
| `%APPDATA%\io.github.quo0.comfyui-organizer\shared-models.yaml` | La configuración generada, en modo opción |
| `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer\` | Instantáneas de nodos por instalación, datos de WebView2 |
| `<biblioteca>\_library.json` | Etiquetas, notas, favoritos — dentro de tu biblioteca, no aquí |

Todo esto es texto plano. Nada sobre tus instalaciones se guarda en ningún
otro sitio, y no se guarda nada en el registro de Windows más allá de la
entrada de desinstalación que crea el instalador.
