; installer\ethernaught.iss

#define MyAppName "ethernaught"
#define MyAppExeName "Ethernaught.exe"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Brad Eagle"
#define MyAppURL "https://ethernaught.net"  ; optional
#define MyDistDir "..\build\dist"

[Setup]
AppId={{7F1C2E94-8A6B-4D73-B5C1-9E0A2F6D4C8B}   ; generate a new GUID once and keep it stable
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
OutputDir=..\build
OutputBaseFilename=Ethernaught-Setup-{#MyAppVersion}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

; If you have an icon, set it here
SetupIconFile=..\res\icon.ico

UninstallDisplayIcon=..\{app}\icon.ico

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
; Copy EVERYTHING from dist into the install directory
Source: "..\res\icon.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#MyDistDir}\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; IconFilename: "{app}\icon.ico"

[Run]
; Launch after install
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent