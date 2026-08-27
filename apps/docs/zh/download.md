---
title: 下载
---

<script setup>
// 数据加载器全站共用一份：发布信息本来就不翻译，构建时也没必要再向
// GitHub 多发一次请求。
import { data } from '../download.data.ts';

const sizeMb = data.size ? (data.size / 1024 / 1024).toFixed(1) : '';
const date = data.publishedAt
  ? new Date(data.publishedAt).toLocaleDateString('zh-CN', {
      year: 'numeric', month: 'long', day: 'numeric',
    })
  : '';
</script>

# 下载

<div v-if="data.available">

**版本 {{ data.version }}** · 发布于 {{ date }} · {{ sizeMb }} MB

<a class="download-button" :href="data.url">下载 {{ data.name }}</a>

Windows 10 或 11，64 位。不需要管理员权限——安装程序会把应用装进你的
用户目录。

</div>
<div v-else>

构建站点时没能读到下载信息。请到发布页面上取最新的安装程序：

<a class="download-button" :href="data.allReleases">打开发布页面</a>

</div>

## 运行之前

Windows 会弹出 **“Windows 已保护你的电脑”** 的提示。这是意料之中的：
本应用没有用代码签名证书签名。
[这个提示长什么样、该怎么处理 →](/zh/guide/smartscreen)

<div v-if="data.available && data.checksums">

想确认文件确实是我们构建的那一份，可以先校验：
<a :href="data.checksums">SHA256SUMS.txt</a>。在 PowerShell 中：

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

正因为文件上没有签名，校验和才是唯一能核对它的办法。

</div>

## 其他版本

- <a :href="data.allReleases">全部发布</a>——包含预发布版本，上面那个按钮
  永远不会指向它们。
- [更新日志](https://github.com/Quo0/ComfyUI-portable-organizer/blob/master/CHANGELOG.md)

安装之后，应用会自己检查更新，并在 **关于** 中提示。这项检查是它唯一
发往你电脑之外的内容，而且可以关掉。详见 [更新应用](/zh/guide/updating)。
