# `instances.json`

El registro de instalaciones. JSON plano, en
`%APPDATA%\io.github.quo0.comfyui-organizer\instances.json`, bajo una única
clave `instances`.

Es una lista de punteros. Nunca contiene modelos, flujos ni nada copiado de
tus instalaciones: solo dónde está cada una y cómo la etiquetaste.

```json
{
  "instances": [
    {
      "id": "i1750000000000",
      "name": "SDXL estable",
      "description": "La que funciona",
      "path": "D:\\AI\\comfy-sdxl",
      "accent": "azure",
      "preferredPort": 8188,
      "comfyVersion": "0.3.62",
      "pythonVersion": "3.12.10",
      "createdAt": 1750000000000,
      "lastStartedAt": 1750600000000,
      "shared": { "enabled": true, "applyMode": "flag" },
      "customProfiles": [
        {
          "id": "custom:1",
          "name": "SDXL, poca VRAM",
          "baseId": "run_nvidia_gpu.bat",
          "args": ["-s", "ComfyUI\\main.py", "--lowvram"]
        }
      ],
      "source": {
        "archivePath": "D:\\downloads\\ComfyUI_windows_portable_nvidia.7z",
        "archiveLabel": "ComfyUI_windows_portable_nvidia.7z",
        "installedAt": 1750000000000
      }
    }
  ]
}
```

## Campos

| Campo | Significado |
| --- | --- |
| `id` | Se genera una vez y nunca se reutiliza. El manifiesto de la biblioteca de flujos lo referencia. |
| `name`, `description` | Tuyos. Nunca se traducen ni se adivinan. |
| `path` | Ruta absoluta a la raíz de la instalación — la carpeta con `python_embeded\` y `ComfyUI\`. |
| `accent` | Un nombre de token de la paleta (`azure`, `moss`, …) o tu propio color como `#rrggbb`. |
| `preferredPort` | Se prueba primero al arrancar; si está ocupado se usa el siguiente libre. |
| `comfyVersion`, `pythonVersion` | Se leen al registrar, solo para mostrarlos. |
| `createdAt`, `lastStartedAt` | Milisegundos desde la época. `lastStartedAt` no existe hasta el primer arranque. |
| `shared` | `enabled` más `applyMode`: `flag` o `instanceFile`. Véase [Modelos compartidos](/es/guide/shared-models#dos-formas-de-aplicarlo). |
| `customProfiles` | Perfiles que construiste encima de uno analizado. `baseId` es el `.bat` que amplían. |
| `source` | Presente cuando la instalación vino del asistente de descompresión. |

Los campos que añaden versiones posteriores de la aplicación llevan valores
por defecto, así que un registro escrito por una versión antigua se lee sin
quejas — y, lo que más importa, sin restablecer nada más de lo que hay
dentro.

`available` no se guarda. Si la carpeta sigue existiendo se recalcula cada
vez que se lee el registro.

## Editarlo a mano

Nada te lo impide, con la aplicación cerrada. Es un archivo pequeño con
rutas absolutas dentro, y arreglar ahí una instalación que se ha mudado es
más rápido que hacer clics. Dos cosas que tener presentes: `path` necesita
las barras invertidas escapadas, como exige JSON, y un `id` referenciado
por tu biblioteca de flujos no conviene cambiarlo a la ligera.
