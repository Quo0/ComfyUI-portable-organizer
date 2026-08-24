# `instances.json`

整合包注册表。纯 JSON，位于
`%APPDATA%\io.github.quo0.comfyui-organizer\instances.json`，全部内容放在
一个 `instances` 键下面。

它是一份指针清单。里面从来不含模型、工作流，也不含任何从你整合包里复制出来
的东西——只有每个整合包在哪儿，以及你给它贴的标签。

```json
{
  "instances": [
    {
      "id": "i1750000000000",
      "name": "SDXL 稳定版",
      "description": "能跑的那个",
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
          "name": "SDXL，低显存",
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

## 字段

| 字段 | 含义 |
| --- | --- |
| `id` | 只生成一次，从不复用。工作流库的清单会引用它。 |
| `name`、`description` | 你的。从不翻译，也从不替你猜。 |
| `path` | 整合包根目录的绝对路径 —— 也就是装着 `python_embeded\` 和 `ComfyUI\` 的那一层。 |
| `accent` | 调色板中的一个名字（`azure`、`moss`……），或者你自己的 `#rrggbb` 颜色。 |
| `preferredPort` | 启动时先试它；被占用就用下一个空闲端口。 |
| `comfyVersion`、`pythonVersion` | 登记时读取，仅用于显示。 |
| `createdAt`、`lastStartedAt` | 自纪元起的毫秒数。首次启动之前不存在 `lastStartedAt`。 |
| `shared` | `enabled` 加上 `applyMode`：`flag` 或 `instanceFile`。参见[共享模型](/zh/guide/shared-models#两种应用方式)。 |
| `customProfiles` | 你在解析出的配置之上做的配置。`baseId` 是它扩展的那个 `.bat`。 |
| `source` | 整合包来自解压向导时才有。 |

后续版本新增的字段都带有默认值，所以旧版本写下的注册表读起来不会有怨言
——更要紧的是，读它的时候不会把里面别的东西一并重置掉。

`available` 不会被存下来。文件夹是否还在，每次读取注册表时都会重新算一遍。

## 手动编辑它

应用关着的时候，没什么拦着你。这是个小文件，里面都是绝对路径，在这儿改一个
搬了家的整合包，比点来点去更快。要记住两件事：`path` 里的反斜杠需要按 JSON
的要求转义，而被你工作流库引用着的 `id`，不要随手改动。
