---
layout: home

hero:
  name: ComfyUI Portable Organizer
  tagline: 轻松管理多个便携版整合包
  image:
    src: /logo.svg
    alt: ComfyUI Portable Organizer
    width: 340
    height: 340
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
  - title: 共享的模型文件夹
    details: 通过 ComfyUI 自带的 extra_model_paths.yaml，让多个整合包指向同一个模型文件夹。20 GB 的大模型不必再变成五个 20 GB。
    link: /zh/guide/shared-models
  - title: 整合包之外的工作流库
    details: 常用的工作流集中放在一个文件夹里，在把它放进某个整合包之前，就能看到那里缺哪些节点。
    link: /zh/guide/workflows
  - title: 你的文件始终是你的
    details: 模型、工作流和整合包本身永远不会被删除——卸载应用时也一样。只有三项操作会碰到你的文件，每一项都由你点击开始。
    link: /zh/guide/uninstall
  - title: 一次安装，铺开多个文件夹
    details: 你下载的 ComfyUI 压缩包只解压一次，就能同时铺到多个文件夹里——每一份都有自己的名称、描述和颜色。向导会替你检查磁盘空间和路径长度，并把新整合包直接接到共享模型文件夹和工作流库上。
    link: /zh/guide/install-from-archive
---

## 它解决什么问题

人们手上会有好几份便携版 ComfyUI，理由很实在：自定义节点彼此冲突，
更新其中一个又会弄坏另一个。于是留下来的，是一堆 `.bat` 文件——
不知道什么装在哪里，端口无从控制，启动日志看不到，每个整合包还要占
一个浏览器标签页。

这个应用就是这些文件夹外面缺的那层壳。它不替代 ComfyUI，不修改它，
也不管理自定义节点——那是 ComfyUI-Manager 的事。

## 为什么用多个便携版，而不是 Comfy Desktop

ComfyUI 真正麻烦的不是模型，而是**自定义节点的依赖**。两个节点可能要求
同一个 Python 库的不同版本，装上或更新其中一个，另一个就坏了。
ComfyUI 的开发者自己也常碰到，公开讨论过：不同自定义节点的依赖集合
本来就不一定兼容。ComfyUI 本身和它的 Python 依赖更新，同样会弄坏昨天
还好好的环境——所以正经项目上，你会想把跑通的那份冻住，另找地方做实验。

便携版回答的正是这个。便携版是一个自成一体的文件夹，有自己的
`python_embeded`、自己的 ComfyUI、自己的依赖。想要几份就留几份：
稳定的一份跑交付的工作流，实验的一份试新版本，再单独留一份给那套
对 torch 有意见的节点。坏了一份，其余的都不会知道。

便携版回答不了的，是打理这些的琐事。好几个文件夹，每个一个 `.bat`，
端口要错开，窗口要盯着，模型和工作流散落在各处——很快就变成
纯手工的例行公事。

这个应用补的就是这一块。它保留便携版擅长的隔离，去掉便携版不擅长的部分。
所有整合包在同一份列表里：启动和停止、分配端口、在标签页里打开、
共用一个模型文件夹和一个工作流库。而整合包本身不会被碰：你的 Python 环境、
你的自定义节点、你的 `.bat` 文件，依旧完全独立。

[Comfy Desktop](https://comfy.org/download) 想把省事和环境管理打包在一起。
这个应用把环境留给你，在便携版之上补上省事的那部分。

## 它不做什么

- 不编辑工作流，也不渲染它们的预览图。
- 不安装、不更新自定义节点。
- 不改动 ComfyUI 自己的主题和语言设置。
- 绝不在整合包之间共享 `custom_nodes`。那会推翻整合包分开存放的全部理由。
