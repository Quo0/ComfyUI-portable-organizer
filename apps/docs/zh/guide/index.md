# 这是什么

ComfyUI Portable Organizer 是一个 Windows 桌面应用，用来管理多份便携版
ComfyUI：整合包的注册表、启动器，以及直接绘制在应用窗口内的 ComfyUI
标签页。

它是你已装好的 ComfyUI 文件外面的一层壳。它从不修改这些文件，从不改写你的
`.bat` 文件，也从不碰 `custom_nodes`。

## 名词

**整合包** — 一个便携版 ComfyUI 文件夹，也就是解压
`ComfyUI_windows_portable_nvidia.7z` 得到的那种。里面有 `python_embeded\`
和 `ComfyUI\`，自成一体。应用里的列表把它们称作**实例**，注册表就是这份
列表。

**启动配置** — 启动一个整合包的一种方式。便携版自带的 `.bat` 文件
（`run_nvidia_gpu`、`run_cpu`、
`advanced\run_nvidia_gpu_disable_api_nodes`……）会各自变成一个配置。你可以
在它们之上添加自己的配置；`.bat` 文件本身原样不动。

**共享模型文件夹** — 整合包之外的一个文件夹，大文件都放在那里。整合包通过
ComfyUI 自带的 `extra_model_paths.yaml` 机制指向它。

**工作流库** — 整合包之外的一个文件夹，放你真正在用的工作流，带标签、备注
和收藏。

## 简短版

1. [安装应用](/zh/guide/install-app)，点过 SmartScreen 提示。
2. [添加整合包](/zh/guide/add-build)——指向已有的文件夹，或者
   [从压缩包解压一份新的](/zh/guide/install-from-archive)。
3. 按 **启动**。应用会挑一个空闲端口，加上 `--disable-auto-launch`
   以免弹出浏览器，并实时显示启动日志，直到服务器响应为止。随后 ComfyUI
   就出现在标签页里。
4. 可选：把整合包连接到[共享模型文件夹](/zh/guide/shared-models)和
   [工作流库](/zh/guide/workflows)。

## 它不会做的事

- 编辑工作流、渲染画布预览，或为工作流做版本管理。
- 安装或更新自定义节点——那是 ComfyUI-Manager 的活儿。应用只会告诉你某个
  工作流需要哪些节点、而这个整合包没有。
- 更改 ComfyUI 自己的主题或语言。
- 在整合包之间共享 `custom_nodes`，永远不会。自定义节点相互冲突，正是分开
  存放整合包的原因。
- 删除你的模型或工作流。哪些会删、哪些不会，完整清单见
  [卸载](/zh/guide/uninstall)。
