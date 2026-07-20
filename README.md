# SZ-1
Алгоритм хеширования: 274 бита лавинного эффекта

## Сборка
Для сборки вам понадобится Rust

### Для Windows (DLL)
```cargo build --release --target=x86_64-pc-windows-msvc```

### Для Linux (SO)
```cargo build --release --target=x86_64-unknown-linux-gnu```

### Для macOS (DYLIB)
```cargo build --release --target=x86_64-apple-darwin```

**После компиляции библиотека появится в target/<*target*>/release**

## Лицензия
Этот проект использует лицензию MIT