@echo off
rem Slow start: forty seconds of progress before readiness.
.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build --cpo-mode slow
pause
