# project path

## add assert_cmd -  aims to simplify the process for doing integration testing of CLIs, including [![alt text][1]](https://crates.io/crates/assert_cmd)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo add assert_cmd
```
<!-- keep the format -->
## Build the project
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo build
```
<!-- kep the format -->
## Run the project
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo run
```
<!-- keep the format -->
## Test the project - With the help of the crate assert_cmd
<!-- keep the format -->
- create a test case
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo test
```
<!-- keep the format -->
## Clean the project
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
#-v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
cargo clean -vv
cargo clean --verbose
Cargo clean
```
<!-- keep the format -->
## Build and run the project
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
#-v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
cargo build -vv # Huge output, very detailed
cargo build -v # Detailed output with compiling commands and flags
cargo build # Standard build
```
<!-- keep the format -->
>&nbsp;[!TIP]
> Tip test
<!-- keep the format -->
>&nbsp;[!TIP] Exclude hidden files and folders in linux find [![alt text][1]](https://stackoverflow.com/questions/58895030/exclude-hidden-files-and-folders-in-linux-find)
><!-- keep the format -->
>```bash <!-- markdownlint-disable-line code-block-style -->
>find . -name "*.md" -not -path '*/[@.]*' -exec grep -nH  "EOF" {} \;
>```
><!-- keep the format -->
<!-- keep the format -->
>&nbsp;[!TIP] How does "cat << EOF" work in bash? [![alt text][1]](https://stackoverflow.com/questions/2500436/how-does-cat-eof-work-in-bash)
> <!-- -->
> ```bash
>cat <<EOF > print.sh
>#!/bin/bash
>echo \$PWD
>echo $PWD
>EOF
>```
><!-- -->
<!-- -->
> [!NOTE]  
> -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="cargo-fmt"' --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("cargo-fmt", "default"))'
<!-- keep the format -->
## Create test of main.rs binary
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
```
<!-- keep the format -->
## Create for your own project a new project folder in the console(terminal ,bash shell), e.g. in your own home folder, and open it as a new project inside your program used - in my case MS VSCode / MS VSCodium
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="rust_main_startup"
echo ${project_name} 
# cd && mkdir <project_name> folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir ${project_name} && cd $_
```
<!-- keep the format -->
>[!TIP]
><!-- keep the format -->
>- Bash Special Variables (\$0,\$?,\$#, \$@, \$\$, \$*, \$-) [![alt text][1]](https://tecadmin.net/bash-special-variables/)
>- Another desc [1](https://stackoverflow.com/questions/5163144/what-are-the-special-dollar-sign-shell-variables)
<!-- -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests
```
<!-- keep the format -->
## Show rustc version verbose
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustc --version --verbose
```
<!-- keep the format -->
>[!TIP]
> Make sure the stable toolchain is activated
<!-- keep the format -->
## Show which toolchain is active
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->
>[!TIP] Markdownlint - Rules inside files can be enabled, disabled
> <!-- markdownlint-disable-next-line --> [![alt text][1]](https://github.com/DavidAnson/markdownlint)
<!-- keep the format -->
## Set/switch  rust toolchain - stable to nightly
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup override set nightly
#or
rustup override set stable
```
<!-- keep the format -->
## Cargo clean - Remove artifacts that cargo has generated in the past
<!-- keep the format -->
- -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
<!-- markdownlint-disable-next-line -->
--color <WHEN>             Coloring [possible values: auto, always, never]
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo clean -vv --color always
```

<!-- -->
<!-- download the link sign -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->
