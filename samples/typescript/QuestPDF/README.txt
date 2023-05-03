
Example commands for Windows:

dotnet publish -r win-x64

PATH=./bin/Release/net8.0/win-x64/publish/:$PATH ~/.deno/bin/deno.exe run --check --allow-ffi --unstable main.ts

