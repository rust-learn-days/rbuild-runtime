init:
	git init
	git add .
	git commit -m "init"
	git branch -M main
	git remote add origin git@github.com:rust-learn-days/rbuild-runtime.git
	pip install pre-commit
	cargo install --locked cargo-deny
	cargo install typos-cli
	cargo install git-cliff
	cargo install cargo-nextest --locked
	pre-commit install
	sed -i '' 's#rust-learn-days/template#rust-learn-days/rbuild-runtime#g' cliff.toml

install-cross:
    cargo install cross --git https://github.com/cross-rs/cross

build:
    rustup target add ${target}
    cargo build --target  ${target}

cross-build:
    rustup target add ${target}
    cross build --target  ${target}

cross-build-release:
    rustup target add ${target}
    cross build --release --target  ${target}
    mkdir -p output/${target}
    cp target/${target}/release/rbuild-runtime output/${target}/rbuild-runtime
    cd output/${target} && tar -zcvf rbuild-runtime-${target}.tar.gz rbuild-runtime

build-release:
    rustup target add ${target}
    cargo build --release --target  ${target}
    mkdir -p output/${target}
    cp target/${target}/release/rbuild-runtime output/${target}/rbuild-runtime
    cd output/${target} && tar -zcvf rbuild-runtime-${target}.tar.gz rbuild-runtime

changelog:
    git-cliff --config cliff.toml > CHANGELOG.md

files:
    wget -O sealos.tar.gz ${url:-https://github.com/labring/sealos/releases/download/v4.3.7/sealos_4.3.7_linux_amd64.tar.gz}
    tar -zxvf sealos.tar.gz sealos && rm -rf sealos.tar.gz
    chmod a+x sealos && mv sealos files/
