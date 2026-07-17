rootdir := ''
prefix := '/usr'
base-dir := absolute_path(clean(rootdir / prefix))
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

bin-src := cargo-target-dir / 'release' / name+ '-applet'
settings-bin-src := cargo-target-dir / 'release' / name + '-settings'
bin-path := 'bin' / name 
bin-dst := base-dir / bin-path + '-applet'
settings-bin-dst := base-dir / bin-path + '-settings'

desktop := APPID + '.desktop'
desktop-src := 'res' / desktop
desktop-path := 'share' / 'applications' / desktop
desktop-dst := base-dir / desktop-path

metainfo := APPID + '.metainfo.xml'
metainfo-src := 'res' / metainfo
metainfo-path := 'share' / 'metainfo' / metainfo
metainfo-dst := base-dir / metainfo-path

icons := APPID + '.svg'
icons-src := 'res' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icons
icons-path := 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icons
icons-dst := base-dir / icons-path

button-icons-src := 'res' / 'icons' / 'bundled' / 'applet-button'

# Compiles with debug profile
build-debug *args:
    cargo build -p {{name}}-applet {{args}}
    cargo build -p {{name}}-settings {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Installs files globally
[no-cd]
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0755 {{settings-bin-src}} {{settings-bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{metainfo-dst}}
    install -Dm0644 {{icons-src}} {{icons-dst}}
    for icon in {{button-icons-src}}/*.svg; do \
        install -Dm0644 "$icon" "{{base-dir}}/share/wmde/{{APPID}}/applet-buttons/$(basename "$icon")"; \
    done


# Uninstalls globally installed files
uninstall:
    rm {{bin-dst}}
    rm {{settings-bin-dst}}
    rm {{desktop-dst}}
    rm {{metainfo-dst}}
    rm {{icons-dst}}
    rm -rf "{{base-dir}}/share/wmde/{{APPID}}"


# Installs files locally
[no-cd]
install-local:
    install -Dm0755 {{bin-src}} ~/.local/bin/{{name}}-applet
    install -Dm0755 {{settings-bin-src}} ~/.local/bin/{{name}}-settings
    install -Dm0644 {{desktop-src}} ${XDG_DATA_HOME:-~/.local/share}/applications/{{desktop}}
    install -Dm0644 {{metainfo-src}} ${XDG_DATA_HOME:-~/.local/share}/metainfo/{{metainfo}}
    install -Dm0644 {{icons-src}} ${XDG_DATA_HOME:-~/.local/share}/icons/hicolor/scalable/apps/{{icons}}
    for icon in {{button-icons-src}}/*.svg; do \
        install -Dm0644 "$icon" "${XDG_DATA_HOME:-~/.local/share}/wmde/{{APPID}}/applet-buttons/$(basename "$icon")"; \
    done

# Uninstalls locally installed files
uninstall-local:
    rm ~/.local/bin/{{name}}-applet
    rm ~/.local/bin/{{name}}-settings
    rm ${XDG_DATA_HOME:-~/.local/share}/applications/{{desktop}}
    rm ${XDG_DATA_HOME:-~/.local/share}/metainfo/{{metainfo}}
    rm ${XDG_DATA_HOME:-~/.local/share}/icons/hicolor/scalable/apps/{{icons}}
    rm -rf "${XDG_DATA_HOME:-~/.local/share}/wmde/{{APPID}}"


# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Vendor dependencies locally
[no-cd]
vendor:
    #!/usr/bin/env bash
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    echo >> .cargo/config.toml
    echo '[env]' >> .cargo/config.toml
    if [ -n "${SOURCE_DATE_EPOCH}" ]
    then
        source_date="$(date -d "@${SOURCE_DATE_EPOCH}" "+%Y-%m-%d")"
        echo "VERGEN_GIT_COMMIT_DATE = \"${source_date}\"" >> .cargo/config.toml
    fi
    if [ -n "${SOURCE_GIT_HASH}" ]
    then
        echo "VERGEN_GIT_SHA = \"${SOURCE_GIT_HASH}\"" >> .cargo/config.toml
    fi
    tar pcf vendor.tar .cargo vendor
    rm -rf .cargo vendor

# Extracts vendored dependencies
[no-cd]
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar
