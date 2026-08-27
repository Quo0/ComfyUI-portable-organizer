@echo off
set CPO_FIXTURE=1
set PY=.\python_embeded\python.exe
rem Launch through a variable: the command itself contains no python.exe,
rem so the parser must give up and fall back to cmd /c.
%PY% -s ComfyUI\main.py --windows-standalone-build --cpo-mode normal
pause
