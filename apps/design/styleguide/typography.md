# Typography

<!-- US-UI-02 · NFR-250 · NFR-260 -->

The system Windows faces are not a saving but what the app will actually be rendered with.

The app exists only on Windows, so the stack starts with `Segoe UI Variable`
and the monospace one with `Cascadia Mono`. The mock-up and the final build
will match character for character. `Microsoft YaHei` in the stack is
mandatory: without it Chinese falls into a random fallback and looks alien. The
base size of 13&nbsp;px suits a tool where the list has to hold many instances
and the console many lines.

## Scale

<ThemePair>
  <div class="scale">
    <div class="scale-row"><span>xl · 22</span><div class="t-xl">Shared models</div></div>
    <div class="scale-row"><span>lg · 18</span><div class="t-lg">Installation wizard</div></div>
    <div class="scale-row"><span>md · 15</span><div class="t-md">SDXL — stable</div></div>
    <div class="scale-row"><span>base · 13</span><div class="t-base">The instance is connected to the shared models folder</div></div>
    <div class="scale-row"><span>sm · 12</span><div class="t-sm">A restart is needed for the change to take effect</div></div>
    <div class="scale-row"><span>label · 11</span><div class="t-label">Launch profile</div></div>
    <div class="scale-row"><span>mono · 12</span><div class="t-mono">D:\AI\SDXL · :8188 · 52.3 GB</div></div>
    <div class="scale-row"><span>CJK</span><div class="t-base">共享模型存储 · 工作流库 · 安装向导</div></div>
  </div>
</ThemePair>

## The startup console · monospace at work

The model-loading progress line is **redrawn in place** rather than appended as
a new one — otherwise a single start swells the log by tens of thousands of
lines (`US-RUN-03/AC-3`).

<ThemePair>
<div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span>
Starting server on 127.0.0.1:8188</div>
</ThemePair>

## Long-string check · section captions

<div class="longform">
  <div class="lf-head">Long-string check · section captions</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Workflow library · Installation wizard · About the app</span></div>
    <div class="lf-row"><b>RU</b><span>Библиотека воркфлоу · Мастер установки · О приложении</span></div>
    <div class="lf-row"><b>ES</b><span>Biblioteca de flujos de trabajo · Asistente de instalación · Acerca de la aplicación</span></div>
    <div class="lf-row"><b>ZH</b><span>工作流库 · 安装向导 · 关于应用</span></div>
  </div>
</div>
