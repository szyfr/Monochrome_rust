@echo off

rem call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsall.bat" x64

cls

for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "date=%dt:~0,4%_%dt:~4,2%_%dt:~6,2%"

set "name=monorust"

xcopy "data\"    "target\debug\%date%\data\" /v /q /s /e /y > nul
xcopy "src\"     "target\debug\%date%\src\"  /v /q /s /e /y > nul

cargo run --target-dir G:\%name%\target\debug\%date%\