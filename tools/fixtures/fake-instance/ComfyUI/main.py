"""A ComfyUI stub for debugging the process side.

Debugging the supervisor against a real build is impossible: a cold start takes
up to five minutes, a crash has to be arranged by hand, and a hang cannot be
reproduced at all. Here all of that is turned on with a flag.

The behaviour is copied from the real ComfyUI where it matters:

* most of the startup goes to **stderr**, not stdout — both have to be read;
* the node loading progress is printed with ``\\r`` and no line feed, the way
  tqdm does it: a naive line collector turns that into thousands of records;
* the line ``To see the GUI go to`` appears at the same moment the server
  becomes ready — it can be used to catch the start without polling the port;
* ``/system_stats`` answers with JSON, and readiness is determined from it.

The modes are selected with ``--cpo-mode``; each has its own ``.bat``, so in the
UI they look like ordinary launch profiles.
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
    """ComfyUI writes its startup to stderr. We do the same, to hit that early."""
    sys.stderr.write(text + "\n")
    sys.stderr.flush()


def out(text: str) -> None:
    sys.stdout.write(text + "\n")
    sys.stdout.flush()


def progress(total: int, seconds: float) -> None:
    """tqdm-style progress: a carriage return with no line feed.

    Exactly the place where the ring log buffer must replace the last line
    instead of appending a new one.
    """
    step = seconds / max(total, 1)
    for i in range(1, total + 1):
        sys.stderr.write(f"\rLoading nodes: {i}/{total}")
        sys.stderr.flush()
        time.sleep(step)
    sys.stderr.write("\n")
    sys.stderr.flush()


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):  # noqa: N802 — the name is dictated by the base class
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
        """Silence the server's own log: it floods the output and skews timing."""


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
    # Flags of the real ComfyUI: what matters is that the stub accepts them and
    # does not crash, otherwise argument mutation before launch is untestable.
    parser.add_argument("--disable-auto-launch", action="store_true")
    parser.add_argument("--windows-standalone-build", action="store_true")
    parser.add_argument("--listen", nargs="?", default=None)
    args, unknown = parser.parse_known_args()
    if unknown:
        err(f"Ignoring unknown args: {' '.join(unknown)}")

    banner(args)

    if args.cpo_mode == "hang":
        # Neither readiness nor a crash: the process lives, the port is free.
        # Exercises the wait timeout and the cancel button.
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
        # Simply holds the port. Needed to check that another port is handed
        # out and that the "port taken" message appears.
        serve(args.port)
        err(f"Holding port {args.port}, not a real server")
        while True:
            time.sleep(3600)

    progress(200, 40.0 if args.cpo_mode == "slow" else 1.5)

    serve(args.port)
    err("Starting server")
    err(f"To see the GUI go to: http://127.0.0.1:{args.port}")

    if args.cpo_mode == "restart":
        # This is how ComfyUI-Manager behaves after installing nodes: it starts
        # a new process and kills the old one. Our handle is lost in the
        # process while the port stays taken — the nastiest scenario.
        time.sleep(8)
        err("[FAKE] restarting myself, the old process is going away")
        # The copy's streams must be detached from ours. An inherited stderr is
        # closed when the parent goes away, and the copy dies on its very first
        # write — which is exactly how this scenario failed the first time.
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
