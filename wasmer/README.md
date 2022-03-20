# Wasmer

## Install
```
$ curl https://get.wasmer.io -sSfL | sh
Welcome to the Wasmer bash installer!

               ww
               wwwww
        ww     wwwwww  w
        wwwww      wwwwwwwww
ww      wwwwww  w     wwwwwww
wwwww      wwwwwwwwww   wwwww
wwwwww  w      wwwwwww  wwwww
wwwwwwwwwwwwww   wwwww  wwwww
wwwwwwwwwwwwwww  wwwww  wwwww
wwwwwwwwwwwwwww  wwwww  wwwww
wwwwwwwwwwwwwww  wwwww  wwwww
wwwwwwwwwwwwwww  wwwww   wwww
wwwwwwwwwwwwwww  wwwww
   wwwwwwwwwwww   wwww
       wwwwwwww
           wwww

downloading: wasmer-darwin-arm64
Latest release: 2.2.1
Downloading archive from https://github.com/wasmerio/wasmer/releases/download/2.2.1/wasmer-darwin-arm64.tar.gz
######################################################################## 100.0%##O=#  #
installing: /Users/masahiro-kondo/.wasmer
Updating bash profile /Users/masahiro-kondo/.zshrc
we've added the following to your /Users/masahiro-kondo/.zshrc
If you have a different profile please add the following:

# Wasmer
export WASMER_DIR="/Users/masahiro-kondo/.wasmer"
[ -s "$WASMER_DIR/wasmer.sh" ] && source "$WASMER_DIR/wasmer.sh"
check: wasmer 2.2.1 installed successfully ✓
wasmer & wapm will be available the next time you open the terminal.
If you want to have the commands available now please execute:

source /Users/masahiro-kondo/.wasmer/wasmer.sh

$ source ~/.wasmer/wasmer.sh
```

## Use wapm

```
$ wapm install cowsay
[INFO] Installing syrusakbary/cowsay@0.2.0
Package installed successfully to wapm_packages!

$ wapm run cowsay hello wapm!
 _____________
< hello wapm! >
 -------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||

$ wapm install saghul/quickjs
[INFO] Installing saghul/quickjs@0.0.3
Package installed successfully to wapm_packages!

$ wapm run quickjs
QuickJS - Type "\h" for help
qjs > [1,2,3,4].map(x => x*x)
[1,2,3,4].map(x => x*x)
[ 1, 4, 9, 16 ]
qjs > 
```

## Use wax

```
$ wax cowsay hello          
 _______
< hello >
 -------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||
```
