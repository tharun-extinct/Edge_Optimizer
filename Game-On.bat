@echo off
echo [INFO] Disabling non-essential background services...
timeout /t 2

:: Kill background processes
taskkill /f /im "HPSystemEventUtilityHost.exe"
taskkill /f /im "HPSystemEventUtilityHost.OSD.exe"
taskkill /f /im "HPCommRecover.exe"
taskkill /f /im "HPOMENSystemOptimizer.exe"
taskkill /f /im "PowerToys.Runner.exe"
taskkill /f /im "PowerToys.AlwaysOnTop.exe"
taskkill /f /im "PowerToys.PowerOCR.exe"
taskkill /f /im "ETDControlCenter.exe"
taskkill /f /im "Widgets.exe"
taskkill /f /im "WidgetService.exe"
taskkill /f /im "BridgeCommunication.exe"
taskkill /f /im "SearchHost.exe"
taskkill /f /im "MicrosoftOfficeSDXHelper.exe"
taskkill /f /im "OMENMQTTBackground.exe"

:: Stop unnecessary services
sc stop DiagTrack
sc stop WSearch
sc stop PcaSvc
sc stop TrkWks
sc stop WdiSystemHost
sc stop StiSvc
sc stop AUEPLauncher
sc stop ETDService
sc stop HPAppHelperCap
sc stop HPDiagsCap
sc stop HPNetworkCap
sc stop HPTouchpointAnalyticsService
sc stop HPPrintScanDoctorService
sc stop HPSysInfoCap
sc stop SECOMNService
sc stop AMDExternalEventsUtility
sc stop AMD Crash Defender Service
sc stop chromoting

echo [INFO] Game mode enabled. You're ready to launch your game!
pause
