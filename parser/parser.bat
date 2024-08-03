@echo off
cd %~dp0\..\dist\welcome-firebase\browser
forfiles /s /m *.js /c "cmd /c %~dp0\\target\\debug\\parser.exe js @path"

%~dp0\\target\\debug\\parser.exe index %~dp0..\dist\welcome-firebase\browser\index.html