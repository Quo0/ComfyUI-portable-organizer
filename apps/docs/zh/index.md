---
layout: home

hero:
  name: ComfyUI Portable Organizer
  text: 多个整合包，<br>一个窗口
  tagline: 便携版 ComfyUI 的注册表、自动挑选空闲端口的启动器，以及直接嵌在应用里的 ComfyUI 标签页——再加上一个共享模型文件夹，取代同一个大模型的五份副本。
  actions:
    - theme: brand
      text: 下载 Windows 版
      link: /zh/download
    - theme: alt
      text: 阅读指南
      link: /zh/guide/
    - theme: alt
      text: GitHub
      link: https://github.com/Quo0/ComfyUI-portable-organizer

features:
  - title: 所有整合包在同一份列表里
    details: 把便携版文件夹指给应用，它会读出 ComfyUI 版本、Python 版本和启动脚本。文件夹里的内容不会被改动。
    link: /zh/guide/add-build
  - title: ComfyUI 就在窗口内
    details: 每个运行中的整合包都有自己的标签页。在两个整合包之间切换只需一次点击，浏览器也不会在你背后弹出来。
    link: /zh/guide/profiles
  - title: 一个模型文件夹
    details: 通过 ComfyUI 自带的 extra_model_paths.yaml，让多个整合包指向同一个模型文件夹。20 GB 的大模型不必再变成五个 20 GB。
    link: /zh/guide/shared-models
  - title: 整合包之外的工作流库
    details: 常用的工作流集中放在一个文件夹里，在把它放进某个整合包之前，就能看到那里缺哪些节点。
    link: /zh/guide/workflows
  - title: 你的文件始终是你的
    details: 模型、工作流和整合包本身永远不会被删除——卸载应用时也一样。只有三项操作会碰到你的文件，每一项都由你点击开始。
    link: /zh/guide/uninstall
  - title: 只支持 Windows，这是有意的
    details: 便携版 ComfyUI 本来就是 Windows 上的做法。安装程序不需要管理员权限，装进你的用户目录。
    link: /zh/guide/install-app
---

## 它解决什么问题

人们手上会有好几份便携版 ComfyUI，理由很实在：自定义节点彼此冲突，
更新其中一个又会弄坏另一个。于是留下来的，是一堆 `.bat` 文件——
不知道什么装在哪里，端口无从控制，启动日志看不到，每个整合包还要占
一个浏览器标签页。

这个应用就是这些文件夹外面缺的那层壳。它不替代 ComfyUI，不修改它，
也不管理自定义节点——那是 ComfyUI-Manager 的事。

## 它不做什么

- 不编辑工作流，也不渲染它们的预览图。
- 不安装、不更新自定义节点。
- 不改动 ComfyUI 自己的主题和语言设置。
- 绝不在整合包之间共享 `custom_nodes`。那会推翻整合包分开存放的全部理由。
