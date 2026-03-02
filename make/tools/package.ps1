param(
  [string]$BuildType   = "release",
  [string]$GtkVersion = "gtk4"
)

#BUILD_TYPE=${1:release}
#GTK_VERSION=${2:gtk4}

$ErrorActionPreference = "Stop"
$AppName = "ethernaught"
$MingwRoot = "C:\msys64\mingw64"
$Out = "..\build\dist"

$exe = Join-Path "..\target\$BuildType" "$AppName.exe"
$bin = Join-Path $MingwRoot "bin"
$share = Join-Path $MingwRoot "share"
$lib = Join-Path $MingwRoot "lib"

if (!(Test-Path $bin)) {
  throw "MSYS2 mingw64 not found at: $bin"
}

Write-Host "== Building release =="

cd ..
#gdk-pixbuf-query-loaders --update-cache
glib-compile-resources res/"$GtkVersion"/windows.gresources.xml --target=res/resources.gresources
cargo build --profile "$BuildType" --no-default-features --features "$GtkVersion"
cd make

if (!(Test-Path $exe)) {
  throw "Release EXE not found: $exe"
}

Write-Host "== Creating $Out =="
Remove-Item -Recurse -Force $Out -ErrorAction SilentlyContinue
New-Item -ItemType Directory $Out | Out-Null

# ------------------------------------------------------------
# Copy EXE
# ------------------------------------------------------------
Copy-Item $exe $Out -Force

# ------------------------------------------------------------
# Copy ALL GTK / MinGW runtime DLLs (THIS IS THE KEY)
# ------------------------------------------------------------
Write-Host "== Copying ALL mingw64 DLLs =="
#Copy-Item "$bin\*.dll" $Out -Force

# ------------------------------------------------------------
# GLib schemas (required)
# ------------------------------------------------------------
#Write-Host "== Copying GLib schemas =="
#$schemaOut = Join-Path $Out "share\glib-2.0\schemas"
#New-Item -ItemType Directory $schemaOut -Force | Out-Null
#Copy-Item (Join-Path $share "glib-2.0\schemas\*") $schemaOut -Recurse -Force
#
#$glibCompile = Join-Path $bin "glib-compile-schemas.exe"
#if (Test-Path $glibCompile) {
#  & $glibCompile $schemaOut | Out-Null
#}

# ------------------------------------------------------------
# gdk-pixbuf loaders + cache (images/icons)
# ------------------------------------------------------------
#Write-Host "== Copying gdk-pixbuf loaders =="
#$pixLoadersSrc = Join-Path $lib "gdk-pixbuf-2.0\2.10.0\loaders"
#$pixLoadersOut = Join-Path $Out "lib\gdk-pixbuf-2.0\2.10.0\loaders"
#New-Item -ItemType Directory $pixLoadersOut -Force | Out-Null
#Copy-Item (Join-Path $pixLoadersSrc "*") $pixLoadersOut -Recurse -Force
#
#Write-Host "== Generating gdk-pixbuf loaders.cache =="
#$pixCacheOutDir = Join-Path $Out "share\gdk-pixbuf-2.0\2.10.0"
#New-Item -ItemType Directory $pixCacheOutDir -Force | Out-Null
#$cachePath = Join-Path $pixCacheOutDir "loaders.cache"
#
#$pixQuery = Join-Path $bin "gdk-pixbuf-query-loaders.exe"
#if (Test-Path $pixQuery) {
#  & $pixQuery --update-cache --output=$cachePath | Out-Null
#}




function Copy-RequiredDlls {
  param(
    [Parameter(Mandatory=$true)][string]$ExePath,
    [Parameter(Mandatory=$true)][string]$OutDir,
    [Parameter(Mandatory=$true)][string]$MingwBin
  )

  $ntldd = Join-Path $MingwBin "ntldd.exe"
  if (!(Test-Path $ntldd)) {
    throw "ntldd.exe not found at $ntldd. Install with: pacman -S mingw-w64-x86_64-ntldd"
  }

  $seen = New-Object 'System.Collections.Generic.HashSet[string]'
  $queue = New-Object 'System.Collections.Generic.Queue[string]'
  $queue.Enqueue((Resolve-Path $ExePath).Path)

  while ($queue.Count -gt 0) {
    $path = $queue.Dequeue()
    if (!$seen.Add($path)) { continue }

    # Get dependencies and pick only DLLs coming from mingw64\bin
    $deps = & $ntldd -R $path 2>$null |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_ -match "=>\s+(.+?\.dll)\s+\(" } |
            ForEach-Object { $matches[1] } |
            Where-Object { $_ -and (Test-Path $_) } |
            ForEach-Object { (Resolve-Path $_).Path } |
            Where-Object { $_.StartsWith((Resolve-Path $MingwBin).Path, [System.StringComparison]::OrdinalIgnoreCase) }

    foreach ($dll in $deps) {
      $dst = Join-Path $OutDir (Split-Path $dll -Leaf)
      if (!(Test-Path $dst)) {
        Copy-Item $dll $dst -Force
        $queue.Enqueue($dll) # recurse: DLLs depend on other DLLs
      }
    }
  }
}

# ... after Copy-Item $exe $Out -Force
Write-Host "== Copying required DLLs (recursive) =="
Copy-RequiredDlls -ExePath (Join-Path $Out "$AppName.exe") -OutDir $Out -MingwBin $bin




# ------------------------------------------------------------
# gdk-pixbuf loaders (PNG/JPG/SVG/etc)
# ------------------------------------------------------------
Write-Host "== Copying gdk-pixbuf loaders =="

$pixRel = "gdk-pixbuf-2.0\2.10.0"
$pixLoadersSrc = Join-Path $MingwRoot ("lib\" + $pixRel + "\loaders")
$pixLoadersOut = Join-Path $Out      ("lib\" + $pixRel + "\loaders")

if (!(Test-Path $pixLoadersSrc)) {
  throw "Pixbuf loaders source not found: $pixLoadersSrc"
}

New-Item -ItemType Directory -Force -Path $pixLoadersOut | Out-Null
Copy-Item (Join-Path $pixLoadersSrc "*") $pixLoadersOut -Recurse -Force

# Copy dependencies for each loader DLL into dist root so they can load
Write-Host "== Copying loader DLL dependencies =="
Get-ChildItem -Path $pixLoadersOut -Filter "*.dll" | ForEach-Object {
  Copy-RequiredDlls -ExePath $_.FullName -OutDir $Out -MingwBin $bin
}

# ------------------------------------------------------------
# Generate loaders.cache in the SAME place you point GDK_PIXBUF_MODULE_FILE
# ------------------------------------------------------------
Write-Host "== Generating gdk-pixbuf loaders.cache =="
$dstToolDir = Join-Path $Out "bin"
$dstTool    = Join-Path $dstToolDir "gdk-pixbuf-query-loaders.exe"
New-Item -ItemType Directory -Force -Path $dstToolDir | Out-Null

$srcTool = Join-Path $MingwRoot "bin\gdk-pixbuf-query-loaders.exe"
Copy-Item $srcTool $dstTool -Force

# Ensure the tool itself can run using your dist DLL set
Copy-RequiredDlls -ExePath $dstTool -OutDir $Out -MingwBin $bin

$env:PATH = "$Out;$Out\bin;$env:PATH"
$env:GDK_PIXBUF_MODULEDIR   = $pixLoadersOut
$env:GDK_PIXBUF_MODULE_FILE = Join-Path $Out ("lib\" + $pixRel + "\loaders.cache")

& $dstTool --update-cache

# Sanity check: print modules found
& $dstTool --print-modules | Out-Host




# ------------------------------------------------------------
# (Optional) Icon themes (uncomment if you use themed icons)
# ------------------------------------------------------------
# Write-Host "== Copying icon themes =="
# $iconsOut = Join-Path $Out "share\icons"
# New-Item -ItemType Directory $iconsOut -Force | Out-Null
# Copy-Item (Join-Path $share "icons\*") $iconsOut -Recurse -Force

Write-Host ""
Write-Host "== DONE =="
Write-Host "Portable build created at: $Out"
Write-Host ""
Write-Host "Test like a clean PC:"
Write-Host "  `$env:PATH='C:\Windows\System32;C:\Windows'"
Write-Host "  Start-Process -WorkingDirectory '$Out' -FilePath '$Out\$AppName.exe'"

& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" ".\build_exe.iss"
