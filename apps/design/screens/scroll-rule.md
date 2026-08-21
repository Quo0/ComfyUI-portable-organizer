# Правило прокрутки

<!-- NFR-420 · NFR-430 · NFR-440 -->

Окно фиксированной высоты, а данные не ограничены: восемь инстансов,
двести воркфлоу, двадцать пять категорий моделей, тысячи строк лога.
Поэтому **окно не прокручивается целиком**. Внутри контентной
области ровно один вертикальный скролл — область данных, а всё, что
управляет экраном, из неё вынесено. Часть экранов ниже показана дважды:
с обычным объёмом и с реальным — второй кадр помечен «**прокрутка**»
и нарисован в окне фиксированной высоты, где данные заведомо не
помещаются.

<div class="two" style="margin-top:16px">
  <div class="paths">
    <div class="path-item"><span class="lbl">Рейл</span><span class="val">закреплён; список запущенных скроллится внутри себя</span></div>
    <div class="path-item"><span class="lbl">Заголовок и действия</span><span class="val">закреплены сверху</span></div>
    <div class="path-item"><span class="lbl">Область данных</span><span class="val">прокручивается</span></div>
    <div class="path-item"><span class="lbl">Подвал мастера</span><span class="val">закреплён снизу</span></div>
  </div>
  <div class="paths">
    <div class="path-item"><span class="lbl">Список — детали</span><span class="val">два независимых скролла, единственное исключение</span></div>
    <div class="path-item"><span class="lbl">Консоль</span><span class="val">следование за лентой с паузой при прокрутке вверх</span></div>
    <div class="path-item"><span class="lbl">Область ComfyUI</span><span class="val">не прокручивается никогда</span></div>
  </div>
</div>
