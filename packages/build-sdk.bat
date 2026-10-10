@echo off
setlocal
set "build_exit=1"

if not exist "%~dp0..\node_modules\.bin\tsc.cmd" (
    echo Cannot find node_modules.
    goto finish
)

echo Wist SDK Compile...
call "%~dp0..\node_modules\.bin\tsc.cmd" -p "%~dp0wist-sdk\tsconfig.json"
set "build_exit=%errorlevel%"

if "%build_exit%"=="0" (
    echo Success : packages\wist-sdk\dist
) else (
    echo Failed.
)

:finish
if /I not "%~1"=="--no-pause" pause
exit /b %build_exit%
