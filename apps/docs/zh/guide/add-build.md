# 添加整合包

打开 **添加**。有两条路：指向你已有的文件夹，或者
[从压缩包解压一份新的](/zh/guide/install-from-archive)。

## 指向已有的文件夹

要选便携版整合包的**根目录**——同时包含 `python_embeded\` 和 `ComfyUI\`
的那一层，而不是其中之一：

```
ComfyUI_windows_portable\      ← 选这个
├─ python_embeded\
│  └─ python.exe
├─ ComfyUI\
│  └─ main.py
├─ run_nvidia_gpu.bat
├─ run_cpu.bat
└─ advanced\
   └─ run_nvidia_gpu_disable_api_nodes.bat
```

应用会检查 `python_embeded\python.exe` 和 `ComfyUI\main.py` 是否都在，
从 `ComfyUI\comfyui_version.py` 读出 ComfyUI 版本，向 `python.exe` 问它
自己的版本，并把 `.bat` 文件收集为[启动配置](/zh/guide/profiles)。

然后你给这个整合包起个名字，可以再写点说明，选一个标识颜色和首选端口。
这四项以后都能改。

## 应用会往整合包目录里写什么

默认情况下：**什么都不写**。注册表放在应用自己的数据目录里，只记住每个
整合包在哪儿。

整合包目录内部最多只会出现两样东西，而且每一样都由你点击开始：

- `ComfyUI\extra_model_paths.yaml`，并且仅当你为共享模型选择了
  [“文件位于整合包内”模式](/zh/guide/shared-models#两种应用方式)；
- 你从[工作流库](/zh/guide/workflows)添加到该整合包里的工作流副本。

这两样都列在 **关于** 页面上，并且在卸载应用后都会留下——它们位于别人的
整合包里，我们无权清理。

## 文件夹不见了，不等于整合包被删了

如果你移动或重命名了整合包目录，注册表里的记录仍在，只是标记为不可用。它
不会悄悄消失——那看上去会像应用弄丢了你的整合包。给它指个新位置，或者把
记录移除。

**移除记录只是移除记录。** 磁盘上的文件夹不会被动。

## 同一版本的两个整合包并排放

同一个压缩包解压两次，或者两个只在自定义节点上有区别的整合包，都可以照样
登记。给它们不同的名字和标识颜色：颜色会跟着整合包出现在侧边栏、卡片和
标签页上，这是区分两个相似整合包最快的办法。
