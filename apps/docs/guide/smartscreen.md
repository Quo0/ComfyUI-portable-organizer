# The SmartScreen warning

When you run the installer, Windows shows a blue box:

> **Windows protected your PC**
> Microsoft Defender SmartScreen prevented an unrecognized app from
> starting. Running this app might put your PC at risk.

The **Run anyway** button is hidden behind **More info**.

## Why it appears

The installer is not signed with a code-signing certificate. SmartScreen
does not know the publisher, so it warns — the same way it warns about any
new program from a small developer.

There is no way around it that costs nothing:

- **Azure Trusted Signing**, the cheap route, is only available to
  organizations and to sole traders registered in the US and Canada.
- A regular OV certificate costs hundreds of dollars a year **and** still
  needs to accumulate reputation before the warning stops appearing.

So the app ships unsigned, and this page exists instead. Hiding that fact
would be worse than the warning itself.

## What to click

1. Click **More info**.
2. Check that the publisher line says *Unknown publisher* and the file name
   is the one you downloaded.
3. Click **Run anyway**.

## How to check the file instead of trusting it

Every release ships a `SHA256SUMS.txt` next to the installer. Compare the
hash of your downloaded file with the one in that file:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

If the two strings match, the file is byte-for-byte the one built by the
release workflow from the public source. If they do not match, delete it
and download again — from [the releases page](https://github.com/Quo0/ComfyUI-portable-organizer/releases)
and nowhere else.

## Updates are signed, though

In-app updates carry a signature of their own, checked before anything is
installed: a `minisign` key pair, with the public half compiled into the
app. An update that does not verify is not installed, and you are told why.

That signature proves the update came from us. It has nothing to do with
SmartScreen — different mechanism, different problem.
