@echo off
chcp 65001 >nul
echo ======================================
echo    Windows Icon Cache Cleaner
echo ======================================
echo.
echo Clearing system icon cache...
ie4uinit.exe -ClearIconCache
echo.

echo Closing Explorer...
taskkill /im explorer.exe /f >nul 2>&1
echo.

echo Deleting icon cache files...
del /f /q "%localappdata%\Microsoft\Windows\Explorer\iconcache_*.db"
echo.

echo Restarting Explorer...
start explorer.exe
echo.
echo ======================================
echo          Clean completed!
echo   Reinstall your app now
echo ======================================
echo.
pause
