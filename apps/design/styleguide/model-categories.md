# Категории моделей

<!-- US-SHARED-02 · NFR-390 -->

Список строится из того, что реально лежит в общей папке, а не из зашитого перечня.

Набор категорий в ComfyUI меняется от версии к версии, поэтому
распознавание идёт по содержимому папки: имена подпапок и есть категории
`NFR-390`. Нераспознанное показывается отдельно, а не
замалчивается. Отдельная строка — **каталог кастомных нод**:
он валиден для этого механизма, но шаринг отменил бы саму причину
разводить инстансы, поэтому исключён безусловно и с объяснением
`US-SHARED-02/AC-4`.

<ThemePair>
  <div class="cats">
    <div class="cat"><code>checkpoints</code><span class="n">14 файлов · 187 ГБ</span><span class="tag">распознано</span></div>
    <div class="cat"><code>loras</code><span class="n">126 файлов · 41 ГБ</span><span class="tag">распознано</span></div>
    <div class="cat"><code>vae</code><span class="n">6 файлов · 2.1 ГБ</span><span class="tag">распознано</span></div>
    <div class="cat"><code>controlnet</code><span class="n">пусто</span><span class="tag">распознано</span></div>
    <div class="cat unknown"><code>my_experiments</code><span class="n">3 файла · 0.4 ГБ</span><span class="tag warn">не распознано</span></div>
    <div class="cat blocked"><code>custom_nodes</code><span class="n">не шарится</span><span class="tag stop">исключено</span></div>
  </div>
</ThemePair>
