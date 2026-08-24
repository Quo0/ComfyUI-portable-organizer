# 启动配置

一个配置，就是一个 `.bat` 文件所表达的意思，只不过换成了应用可以直接启动
的形式。配置在每次启动时都从磁盘重新推导：`.bat` 文件才是事实来源，而且
它们从不被修改。

## 它长什么样

| 字段 | 含义 |
| --- | --- |
| `id` | `.bat` 相对于整合包根目录的路径。它也是这个配置的身份：`run_nvidia_gpu.bat`、`advanced\run_nvidia_gpu_disable_api_nodes.bat`。 |
| `name` | 去掉扩展名的文件名。从不翻译。 |
| `advanced` | 它来自 `advanced\` 子文件夹。 |
| `pythonPath` | 那一行所调用的解释器的绝对路径。 |
| `args` | 参数，已切分为词，并尊重引号。 |
| `cwd` | `.bat` 文件自己所在的文件夹。 |
| `env` | 命令上方那些 `set KEY=VALUE` 行里的变量。 |
| `fallback` | 解析失败；该文件将改用 `cmd /c` 运行。 |

你自己的配置会沿用基础配置的一切，只替换 `args` —— 参见
[`instances.json`](/zh/reference/instances-json)。

## 这个文件是怎么读的

`echo`、`pause`、`rem` 和空行都会跳过。`set KEY=VALUE` 行变成环境变量。
第一条真正的命令行按尊重引号的方式切分成词，可执行文件成为 `pythonPath`，
其余成为 `args`。

相对路径——包括 `advanced\*.bat` 里的 `..\`——都以那个 `.bat` 文件所在的
文件夹为基准解析，因为双击它时就是这样。那个文件夹同时也成为工作目录。

不会拉起服务器的配置，比如包里自带的更新脚本，不会进入启动列表。

## `fallback: true`

某一行解析不了时，配置不会被丢弃：它被标记为回退，改用 `cmd /c` 启动。
界面上会写明这一点，因为这条路径在应用和 Python 之间多塞了一个
`cmd.exe`，会让停止进程变得不那么可靠。

## 从参数推导出来的路径

有三个文件夹不是想当然定下来的，而是从参数里解析出来的，遵循的是 ComfyUI
自己的优先级：

| 文件夹 | 依次查看的参数 |
| --- | --- |
| 模型 | `--base-directory`，否则 `<整合包>\ComfyUI\models` |
| 用户数据（工作流在这里） | `--user-directory`，然后 `--base-directory`，否则 `<整合包>\ComfyUI\user` |
| 输出 | `--output-directory`，然后 `--base-directory`，否则 `<整合包>\ComfyUI\output` |

所以即便你的配置把这些文件夹挪到了别处，“打开输出文件夹”和“把工作流添加到
已停止的整合包”也仍然落在正确的位置上。
