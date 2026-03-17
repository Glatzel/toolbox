$current_dir=Resolve-Path $PWD
Set-Location $PSScriptRoot
Set-Location ../..
Set-Location bin
function split_line($title) {
        $terminalWidth = [System.Console]::WindowWidth
        $separator = '=' * $terminalWidth
        Write-Output $separator
        Write-Output $title
        Write-Output $separator
}
# vinaya
split_line -title vinaya
./vinaya.exe houdini --help

# vinya houdini
function vinaya_houdini {
        split_line -title "vinaya houdini"

        # generate proto headers
        ./vinaya.exe -vvv houdini generate-proto `
                --major 20 `
                --minor 5 `
                --patch 445 `
                --python-version-minor 11 `
                -i ../vinaya-cpp/src/SOP_Star/SOP_Star.C `
                -o ../temp/cli/example.proto.h

        # HFS
        Write-Host -NoNewline "Houdini 20.5.456 HFS: "
        ./vinaya.exe houdini hfs from-version --major 20 --minor 5 --patch 456 --no-check
        Write-Host -NoNewline "Houdini 20.5.123 HFS: "
        ./vinaya.exe houdini hfs from-version-string "20.5.123" --no-check
        Write-Host -NoNewline "Houdini latest HFS: "
        ./vinaya.exe houdini hfs latest

        # latest-installed-version
        Write-Host -NoNewline "Houdini latest version major: "
        ./vinaya.exe houdini latest major
        Write-Host -NoNewline "Houdini latest version major: "
        ./vinaya.exe houdini latest minor
        Write-Host -NoNewline "Houdini latest version major: "
        ./vinaya.exe houdini latest patch
        Write-Host -NoNewline "Houdini latest version: "
        ./vinaya.exe houdini latest version
        Write-Host -NoNewline "Houdini latest version without patch: "
        ./vinaya.exe houdini latest version --no-patch
        Write-Host ""

        # list-installed
        Write-Host "List installed houdini version: "
        ./vinaya.exe houdini list

        Write-Host ""
}
vinaya_houdini

# # preference-directory
Write-Host -NoNewline "Houdini 20.5 preference directory: "
./vinaya.exe preference-directory --major 20 --minor 5 --no-check
Write-Host ""

# package
function vinaya_package {
        split_line -title "vinaya sidefx"
        Write-Host -NoNewline "Houdini 20.5 packages directory: "
        ./vinaya.exe package --major 20 --minor 5 dir


        Write-Host "Houdini 20.5 packages:"
        ./vinaya.exe package --major 20 --minor 5 list
        Write-Host ""
}

# sidefx
function vinaya_sidefx {
        split_line -title "vinaya sidefx"
        & ./vinaya.exe sidefx `
                download.get-daily-builds-list `
                --product houdini-launcher `
                --major 20 `
                --minor 5 `
                --platform win64 `
        | jq '.[:2]| .[] | {build,date}'

        ./vinaya.exe sidefx `
                download.get-daily-build-download `
                --product houdini-launcher `
                --major 20 `
                --minor 5 `
                --build production `
                --platform win64 | jq '.download_url'
}
vinaya_sidefx

Set-Location $current_dir
