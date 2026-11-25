[Setup]
AppName=Sta. Clara Dialysis Center Inventory
AppVersion=1.0.0
AppPublisher=Sta. Clara Dialysis Center, Inc.
DefaultDirName={autopf}\StaClaraDialysis
DefaultGroupName=Sta. Clara Dialysis Center
OutputDir=..\dist
OutputBaseFilename=StaClaraDialysis-Setup
SetupIconFile=..\assets\logo.ico
UninstallDisplayIcon={app}\StaClaraDialysis.exe
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "..\dist\StaClaraDialysis.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\sample_excel\Inventory-NOV-16-23 (1).xlsx"; DestDir: "{app}\sample_excel"; Flags: ignoreversion

[Icons]
Name: "{group}\Sta. Clara Dialysis Center"; Filename: "{app}\StaClaraDialysis.exe"; IconFilename: "{app}\StaClaraDialysis.exe"
Name: "{autodesktop}\Sta. Clara Dialysis Center"; Filename: "{app}\StaClaraDialysis.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\StaClaraDialysis.exe"; Description: "{cm:LaunchProgram,Sta. Clara Dialysis Center}"; Flags: nowait postinstall skipifsilent
