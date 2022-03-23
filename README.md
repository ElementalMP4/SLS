# Simple LS

An LS command clone that shows all files and folders in a directory, as well as file sizes with colour coding.

## Using SLS

First, ensure that `sls` is in a location on PATH. For Windows, `C:\Windows\System32` is already on PATH. For Linux, try `/usr/local/bin` 

When using SLS you will be presented with items that are two different colours:

`Red = file`

`Yellow = folder`

Files follow this format: `(filename - Size)`
Folders follow this format: `folder name`

## Compiling SLS

SLS requires C++ 17. If using G++ be sure to use the `-std=c++17` flag.

Example Windows build command: `g++ .\SLS.cpp -o sls.exe -std=c++17 -s -Os`
Example Linux build command: `g++ .\SLS.cpp -o sls -std=c++17 -s -Os`