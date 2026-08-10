"""Заглушка ComfyUI для отладки процессной части.

Отлаживать супервизор на реальной сборке невозможно: холодный старт идёт
до пяти минут, падение приходится подстраивать руками, а зависание
не воспроизвести вовсе. Здесь всё это включается флагом.

Поведение скопировано с настоящего ComfyUI там, где это важно:

* основная часть старта идёт в **stderr**, а не в stdout — читать надо оба;
* прогресс загрузки нод печатается через ``\\r`` без перевода строки,
  как это делает tqdm: наивный сборщик строк превратит его в тысячи записей;
* строка ``To see the GUI go to`` появляется одновременно с готовностью
  сервера — по ней можно ловить старт, не дожидаясь опроса порта;
* ``/system_stats`` отвечает JSON-ом, по нему определяется готовность.

Режимы задаются ``--cpo-mode``; каждому соответствует свой ``.bat``,
поэтому в интерфейсе они выглядят как обычные профили запуска.
"""

import argparse
import json
import os
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

MODES = ("normal", "slow", "crash", "hang", "restart", "hold")


def err(text: str) -> None:
    """ComfyUI пишет старт в stderr. Повторяем, чтобы поймать эту ошибку рано."""
    sys.stderr.write(text + "\n")
    sys.stderr.flush()


def out(text: str) -> None:
    sys.stdout.write(text + "\n")
    sys.stdout.flush()


def progress(total: int, seconds: float) -> None:
    """Прогресс в стиле tqdm: возврат каретки без перевода строки.

    Ровно то место, где кольцевой буфер логов обязан заменять последнюю
    строку, а не добавлять новую.
    """
    step = seconds / max(total, 1)
    for i in range(1, total + 1):
        sys.stderr.write(f"\rLoading nodes: {i}/{total}")
        sys.stderr.flush()
        time.sleep(step)
    sys.stderr.write("\n")
    sys.stderr.flush()


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):  # noqa: N802 — имя задано базовым классом
        if self.path.startswith("/system_stats"):
            body = json.dumps(
                {
                    "system": {"comfyui_version": "0.0.0-fake", "python_version": sys.version},
                    "devices": [{"name": "fake", "vram_total": 0}],
                }
            ).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return

        body = b"<html><head><title>ComfyUI (fake)</title></head><body>fake</body></html>"
        self.send_response(200)
        self.send_header("Content-Type", "text/html")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *_args):
        """Гасим собственный лог сервера: он забивает вывод и мешает замеру."""


def serve(port: int) -> ThreadingHTTPServer:
    server = ThreadingHTTPServer(("127.0.0.1", port), Handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server


def banner(args) -> None:
    err("[START] Security scan")
    err(f"Fake ComfyUI, mode={args.cpo_mode}, pid={os.getpid()}")
    out("Total VRAM 24576 MB, total RAM 65536 MB")
    err("Set vram state to: NORMAL_VRAM")


def main() -> int:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("--port", type=int, default=8188)
    parser.add_argument("--cpo-mode", choices=MODES, default="normal")
    # Флаги настоящего ComfyUI: важно, что заглушка их принимает и не падает,
    # иначе не проверить мутацию аргументов перед стартом.
    parser.add_argument("--disable-auto-launch", action="store_true")
    parser.add_argument("--windows-standalone-build", action="store_true")
    parser.add_argument("--listen", nargs="?", default=None)
    args, unknown = parser.parse_known_args()
    if unknown:
        err(f"Ignoring unknown args: {' '.join(unknown)}")

    banner(args)

    if args.cpo_mode == "hang":
        # Ни готовности, ни падения: процесс живёт, порт не занят.
        # Проверяет таймаут ожидания и кнопку отмены.
        err("Loading nodes: 1/200")
        while True:
            time.sleep(3600)

    if args.cpo_mode == "crash":
        progress(20, 1.5)
        err("Traceback (most recent call last):")
        err('  File "main.py", line 1, in <module>')
        err("RuntimeError: fake crash on purpose")
        return 1

    if args.cpo_mode == "hold":
        # Просто занимает порт. Нужен, чтобы проверить выдачу другого порта
        # и сообщение о занятом.
        serve(args.port)
        err(f"Holding port {args.port}, not a real server")
        while True:
            time.sleep(3600)

    progress(200, 40.0 if args.cpo_mode == "slow" else 1.5)

    serve(args.port)
    err("Starting server")
    err(f"To see the GUI go to: http://127.0.0.1:{args.port}")

    if args.cpo_mode == "restart":
        # Так ведёт себя ComfyUI-Manager после установки нод: он поднимает
        # новый процесс и гасит старый. Наш хэндл при этом теряется,
        # а порт остаётся занятым — самый неприятный сценарий.
        time.sleep(8)
        err("[FAKE] restarting myself, the old process is going away")
        # Потоки копии обязаны быть отвязаны от наших. Унаследованный stderr
        # закрывается вместе с уходом родителя, и копия падает на первой же
        # записи — ровно этим сценарий и провалился в первый раз.
        subprocess.Popen(
            [sys.executable, os.path.abspath(__file__), "--port", str(args.port),
             "--cpo-mode", "normal"],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            creationflags=getattr(subprocess, "DETACHED_PROCESS", 0),
        )
        time.sleep(0.5)
        return 0

    while True:
        time.sleep(3600)


if __name__ == "__main__":
    sys.exit(main())
