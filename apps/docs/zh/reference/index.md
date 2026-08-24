# 参考

应用写了什么、写到哪里，以及交给 ComfyUI 的是什么。这一部分的存在，是为了
让你能核查这个应用，而不是只能信任它。

- [我们添加的参数](/zh/reference/flags) —— 那三个命令行参数，以及每一个
  为什么在那里。
- [instances.json](/zh/reference/instances-json) —— 整合包注册表的格式。
- [启动配置](/zh/reference/launch-profile) —— 一个 `.bat` 文件怎么变成
  配置。
- [生成的 YAML](/zh/reference/extra-model-paths) —— 我们产出的共享模型
  配置。

## 各样东西放在哪儿

| 路径 | 内容 |
| --- | --- |
| `%APPDATA%\io.github.quo0.comfyui-organizer\settings.json` | 设置：主题、语言、共享模型、工作流库路径 |
| `%APPDATA%\io.github.quo0.comfyui-organizer\instances.json` | 整合包注册表 |
| `%APPDATA%\io.github.quo0.comfyui-organizer\shared-models.yaml` | 参数模式下生成的配置 |
| `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer\` | 各整合包的节点快照、WebView2 数据 |
| `<库>\_library.json` | 标签、备注、收藏 —— 在你的库文件夹里，不在这儿 |

这些全是纯文本。关于你整合包的信息不会存在别的任何地方，除了安装程序创建
的卸载项之外，Windows 注册表里也不存任何东西。
