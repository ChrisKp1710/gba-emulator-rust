# Installazione SDL2 per Windows

## Opzione 1: Download manuale (Consigliato)

1. Scarica SDL2 development libraries per MSVC:
   https://github.com/libsdl-org/SDL/releases/latest
   
   Cerca il file: `SDL2-devel-X.X.X-VC.zip`

2. Estrai l'archivio

3. Copia i file nella cartella del progetto:
   ```powershell
   # Dalla directory SDL2-X.X.X\
   Copy-Item -Path "lib\x64\SDL2.lib" -Destination "C:\Users\chris\Documents\gba-emulator-rust\SDL2.lib"
   Copy-Item -Path "lib\x64\SDL2.dll" -Destination "C:\Users\chris\Documents\gba-emulator-rust\SDL2.dll"
   ```

4. Imposta la variabile d'ambiente:
   ```powershell
   $env:SDL2_LIB_DIR = "C:\Users\chris\Documents\gba-emulator-rust"
   ```

## Opzione 2: vcpkg (Automatico)

1. Installa vcpkg se non è già installato:
   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   ```

2. Installa SDL2:
   ```powershell
   .\vcpkg install sdl2:x64-windows
   ```

3. Imposta CMAKE_TOOLCHAIN_FILE:
   ```powershell
   $env:CMAKE_TOOLCHAIN_FILE = "C:\path\to\vcpkg\scripts\buildsystems\vcpkg.cmake"
   ```

## Opzione 3: Compilazione senza SDL2

Per testare senza SDL2, puoi temporaneamente disabilitare il frontend:

```powershell
# Build solo il core
cargo build --release -p gba-core
cargo build --release -p gba-arm7tdmi
```

## Verifica installazione

```powershell
cargo build --release
```

Se la compilazione ha successo, vedrai l'eseguibile in:
`target\release\gba-emulator.exe`

## Note

- Su Linux: `sudo apt-get install libsdl2-dev`
- Su macOS: `brew install sdl2`
