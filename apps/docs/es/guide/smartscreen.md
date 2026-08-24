# El aviso de SmartScreen

Al ejecutar el instalador, Windows muestra un recuadro azul:

> **Windows protegió su PC**
> Microsoft Defender SmartScreen impidió el inicio de una aplicación
> desconocida. Si ejecuta esta aplicación, podría poner en riesgo su PC.

El botón **Ejecutar de todas formas** está escondido detrás de
**Más información**.

## Por qué aparece

El instalador no está firmado con un certificado de firma de código.
SmartScreen no conoce al editor, así que avisa — igual que avisa de
cualquier programa nuevo de un desarrollador pequeño.

No hay forma de evitarlo que salga gratis:

- **Azure Trusted Signing**, la vía barata, solo está disponible para
  organizaciones y para autónomos registrados en EE. UU. y Canadá.
- Un certificado OV normal cuesta cientos de dólares al año **y** aun así
  necesita acumular reputación antes de que el aviso deje de aparecer.

Así que la aplicación se publica sin firmar y, en lugar de la firma, existe
esta página. Ocultar el hecho sería peor que el propio aviso.

## Qué pulsar

1. Pulsa **Más información**.
2. Comprueba que la línea del editor dice *Editor desconocido* y que el
   nombre del archivo es el que descargaste.
3. Pulsa **Ejecutar de todas formas**.

## Cómo comprobar el archivo en vez de confiar en él

Cada publicación incluye un `SHA256SUMS.txt` junto al instalador. Compara
el hash del archivo que descargaste con el que figura ahí:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

Si las dos cadenas coinciden, el archivo es byte a byte el que compiló el
flujo de publicación a partir del código público. Si no coinciden,
bórralo y descárgalo otra vez — desde
[la página de publicaciones](https://github.com/Quo0/ComfyUI-portable-organizer/releases)
y desde ningún otro sitio.

## Las actualizaciones sí van firmadas

Las actualizaciones dentro de la aplicación llevan una firma propia, que se
verifica antes de instalar nada: un par de claves `minisign`, con la mitad
pública compilada dentro de la aplicación. Una actualización que no
verifica no se instala, y se te dice por qué.

Esa firma demuestra que la actualización viene de nosotros. No tiene nada
que ver con SmartScreen: mecanismo distinto, problema distinto.
