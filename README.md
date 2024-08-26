# snilsgen
Консольная утилита для генерирования валидных значений СНИЛС.

## Общее описание

СНИЛС — страховой номер индивидуального лицевого счёта, который присваивается каждому гражданину РФ.

Формат СНИЛС: «ХХХ-ХХХ-ХХХ YY», где X — девять цифр самого номера, Y — две цифры контрольной суммы, 
вычисляемой по особому алгоритму из последовательности первых цифр.

## Алгоритм формирования

1. Случайным образом генерируется номер больше номера 001-001-998
2. В полученном номере не может присутствовать одна и та же цифра три раза подряд. Дефисы при этой 
проверке игнорируются. Например, неверными будут все нижеследующие номера:
XXX-555-XXX
XX7-77X-XXX
3. Контрольное число рассчитывается следующим образом:
	1. Каждая цифра СНИЛС умножается на номер своей позиции (позиции отсчитываются с конца, то есть, справа)
	2. Полученные произведения суммируются
	3. Берётся остаток от деления на 101 (если получилось 100, то контрольное число равно 00)

## Установка

```
cargo install --git https://github.com/alturus/snilsgen snilsgen
```

## Использование

```
$ snilsgen [OPTIONS]
```

### Необязательные параметры

```
 -c, --clipboard  скопировать полученное значение в буфер обмена

 -h, --help       отобразить подсказку по использованию
 -V, --version    отобразить версию утилиты
```

### Пример

```
$ snilsgen -c
733-548-365 71
Snils copied to clipboard.
```
