# 安装应用

## 需要什么

- Windows 10 或 11，64 位。
- WebView2 运行时。Windows 11 和较新的 Windows 10 都自带；如果缺少，安装
  程序会顺带装上。
- 至少一份便携版 ComfyUI 整合包——或者一个压缩包，应用可以替你解压。

不需要管理员权限。

## 安装过程

1. [下载安装程序](/zh/download)。
2. Windows 会显示 **“Windows 已保护你的电脑”**。这是意料之中的，也不是病毒
   告警——该点什么、怎么校验文件，见
   [SmartScreen 提示页](/zh/guide/smartscreen)。
3. 应用装进你的用户目录
   （`%LOCALAPPDATA%\Programs\ComfyUI Portable Organizer`）并启动。

## 应用把自己的数据放在哪里

| 内容 | 位置 |
| --- | --- |
| 设置、整合包注册表、工作流库路径 | `%APPDATA%\io.github.quo0.comfyui-organizer` |
| 缓存与 WebView2 浏览器数据 | `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer` |

这两个文件夹都列在应用的 **关于** 页面里，每个旁边都有一个在资源管理器中
打开的按钮。`文档`、`%PROGRAMDATA%` 以及可执行文件旁边，都不会写入任何
东西。

你的模型、工作流和 ComfyUI 整合包放在哪里，就还在哪里，应用既不会把它们
搬过去，也不会搬走。参见[卸载](/zh/guide/uninstall)。

## 首次启动

应用打开时注册表是空的，要做的只有一件事：添加整合包。首次启动的语言跟随
Windows——英语、俄语、简体中文和西班牙语都已内置——主题跟随系统，直到你在
**设置 → 外观** 里选定为止。
