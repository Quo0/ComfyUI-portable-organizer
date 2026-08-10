@echo off
set CPO_FIXTURE=1
set PY=.\python_embeded\python.exe
rem Запуск через переменную: строки с python.exe в самой команде нет,
rem поэтому парсер обязан сдаться и откатиться на cmd /c.
%PY% -s ComfyUI\main.py --windows-standalone-build --cpo-mode normal
pause
