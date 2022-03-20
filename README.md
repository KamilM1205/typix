# Typix

## Что такое typix

Typix - это игра для тренеровки слепой печати написанно на языке Rust без использования игровых движков. Доступна для Linux/Windows/MacOS

## Перед компиляцией и запуском typix

### Установка компилятора Rust

Для установки компилятора раст перейдите на [оф. сайт](https://www.rust-lang.org/tools/install) и следуйте инструкции для вашей платформы.

### Для linux нужно установить зависимости

```bash

# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib

```

## Сборка и запуск typix

```bash

git clone https://github.com/typix
cd typix
cargo run --release

```

## О проекте

Проект создан специально для конкурса MasterIT 2022.
