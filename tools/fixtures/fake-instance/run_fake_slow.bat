@echo off
rem Долгий старт: сорок секунд прогресса перед готовностью.
.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build --cpo-mode slow
pause
