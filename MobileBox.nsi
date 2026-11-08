; MobileBox NSIS Installer Script
!include "MUI2.nsh"

Name "MobileBox"
OutFile "MobileBox-Setup.exe"
InstallDir "$LOCALAPPDATA\MobileBox"
InstallDirRegKey HKCU "Software\MobileBox" "InstallDir"
RequestExecutionLevel user

!define MUI_ICON "${NSDIR}\MobileBox.ico"
!define MUI_UNICON "${NSDIR}\MobileBox.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

Section "MainSection" SEC01
  SetOutPath "$INSTDIR"
  File "MobileBox.exe"
  File "mobilebox-mcp.exe"
  File "MobileBox.ico"
  File "MobileBox-Hotkey.ps1"
  File "MobileBox-Settings.ps1"
  File "scripts\setup.ps1"
  File "scripts\setup.sh"
  File /r "skins"
  File "README.md"
  File "LICENSE"
  File /r "docs"

  CreateDirectory "$SMPROGRAMS\MobileBox"
  CreateShortcut "$SMPROGRAMS\MobileBox\MobileBox.lnk" "$INSTDIR\MobileBox.exe"
  CreateShortcut "$DESKTOP\MobileBox.lnk" "$INSTDIR\MobileBox.exe"

  WriteRegStr HKCU "Software\MobileBox" "InstallDir" "$INSTDIR"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\*.*"
  RMDir /r "$INSTDIR"
  Delete "$SMPROGRAMS\MobileBox\*.*"
  RMDir "$SMPROGRAMS\MobileBox"
  Delete "$DESKTOP\MobileBox.lnk"
  DeleteRegKey HKCU "Software\MobileBox"
SectionEnd