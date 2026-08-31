# The scrolling rule

<!-- NFR-420 · NFR-430 · NFR-440 -->

The window has a fixed height and the data does not: eight instances, two
hundred workflows, twenty-five model categories, thousands of log lines. So
**the window never scrolls as a whole**. Inside the content area there is
exactly one vertical scroll — the data area, with everything that controls the
screen lifted out of it. Some of the screens below are shown twice: with an
ordinary amount of data and with a realistic one — the second frame is marked
"**scrolling**" and drawn in a window of fixed height where the data plainly
does not fit.

<div class="two" style="margin-top:16px">
  <div class="paths">
    <div class="path-item"><span class="lbl">Rail</span><span class="val">pinned; the list of running instances scrolls within itself</span></div>
    <div class="path-item"><span class="lbl">Heading and actions</span><span class="val">pinned at the top</span></div>
    <div class="path-item"><span class="lbl">Data area</span><span class="val">scrolls</span></div>
    <div class="path-item"><span class="lbl">Wizard footer</span><span class="val">pinned at the bottom</span></div>
  </div>
  <div class="paths">
    <div class="path-item"><span class="lbl">List — details</span><span class="val">two independent scrolls, the only exception</span></div>
    <div class="path-item"><span class="lbl">Console</span><span class="val">follows the tail, pausing when scrolled up</span></div>
    <div class="path-item"><span class="lbl">ComfyUI area</span><span class="val">never scrolls</span></div>
  </div>
</div>
