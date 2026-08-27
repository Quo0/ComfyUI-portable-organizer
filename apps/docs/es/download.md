---
title: Descargar
---

<script setup>
// El cargador de datos es uno para todo el sitio: los datos de la
// publicación no se traducen y una segunda petición a GitHub durante la
// compilación no hace falta.
import { data } from '../download.data.ts';

const sizeMb = data.size ? (data.size / 1024 / 1024).toFixed(1) : '';
const date = data.publishedAt
  ? new Date(data.publishedAt).toLocaleDateString('es-ES', {
      year: 'numeric', month: 'long', day: 'numeric',
    })
  : '';
</script>

# Descargar

<div v-if="data.available">

**Versión {{ data.version }}** · publicada el {{ date }} · {{ sizeMb }} MB

<a class="download-button" :href="data.url">Descargar {{ data.name }}</a>

Windows 10 u 11, 64 bits. No hacen falta permisos de administrador: el
instalador coloca la aplicación en tu perfil de usuario.

</div>
<div v-else>

Los datos de la descarga no se pudieron leer al compilar el sitio. Coge el
instalador más reciente en la página de publicaciones:

<a class="download-button" :href="data.allReleases">Abrir la página de publicaciones</a>

</div>

## Antes de ejecutarlo

Windows mostrará el aviso **«Windows protegió su PC»**. Es lo esperado: la
aplicación no está firmada con un certificado de firma de código.
[Qué aspecto tiene el aviso y qué hacer con él →](/es/guide/smartscreen)

<div v-if="data.available && data.checksums">

Si quieres asegurarte de que el archivo es el que compilamos nosotros,
verifícalo antes: <a :href="data.checksums">SHA256SUMS.txt</a>. En
PowerShell:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

La suma de comprobación es la única forma de comprobar el archivo,
precisamente porque no lleva firma.

</div>

## Otras versiones

- <a :href="data.allReleases">Todas las publicaciones</a> — incluidas las
  preliminares, a las que el botón de arriba no apunta nunca.
- [Registro de cambios](https://github.com/Quo0/ComfyUI-portable-organizer/blob/master/CHANGELOG.md)

Una vez instalada, la aplicación busca actualizaciones por su cuenta y las
ofrece en **Acerca de**. Esa comprobación es lo único que sale de tu
ordenador, y se puede desactivar. Véase
[Actualizar](/es/guide/updating).
