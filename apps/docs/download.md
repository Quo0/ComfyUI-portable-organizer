---
title: Download
---

<script setup>
import { data } from './download.data.ts';

const sizeMb = data.size ? (data.size / 1024 / 1024).toFixed(1) : '';
const date = data.publishedAt
  ? new Date(data.publishedAt).toLocaleDateString('en-GB', {
      year: 'numeric', month: 'long', day: 'numeric',
    })
  : '';
</script>

# Download

<div v-if="data.available">

**Version {{ data.version }}** · released {{ date }} · {{ sizeMb }} MB

<a class="download-button" :href="data.url">Download {{ data.name }}</a>

Windows 10 or 11, 64-bit. No administrator rights needed — the installer
puts the app into your user profile.

</div>
<div v-else>

The download details could not be read at build time. Pick the newest
installer on the releases page:

<a class="download-button" :href="data.allReleases">Open the releases page</a>

</div>

## Before you run it

Windows will show a **"Windows protected your PC"** warning. That is
expected: the app is not signed with a code-signing certificate.
[What the warning looks like and what to do about it →](/guide/smartscreen)

<div v-if="data.available && data.checksums">

Verify the file first if you want to be sure it is the one we built:
<a :href="data.checksums">SHA256SUMS.txt</a>. In PowerShell:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

The checksum is the only way to check the file, precisely because there is
no signature on it.

</div>

## Other versions

- <a :href="data.allReleases">All releases</a> — including pre-releases,
  which the button above never points at.
- [Changelog](https://github.com/Quo0/ComfyUI-portable-organizer/blob/master/CHANGELOG.md)

Once installed, the app checks for updates itself and offers them in
**About**. That check is the only thing it ever sends outside your
computer, and it can be turned off. See [Updating](/guide/updating).

<style>
.download-button {
  display: inline-block;
  margin: 1rem 0;
  padding: 0.6rem 1.4rem;
  border-radius: 20px;
  background: var(--vp-c-brand-3);
  color: var(--vp-c-white);
  font-weight: 600;
  text-decoration: none;
}
.download-button:hover {
  background: var(--vp-c-brand-2);
}
</style>
