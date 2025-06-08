# [PWNABLE.KR](http://pwnable.kr/play.php)

## [fd]

`fd` takes a number `x` as input, reads `x - 0x1234` into `buf` and if `buf` == `LETMEWIN\n`, it prints the flag

if we make fd == 0, we can pass the string `LETMEWIN\n` via stdin 

0 = stdin
1 = stdout
2 = stderr

```
echo "LETMEWIN" | ./fd 4660
```

## [collision]

```c
strlen(argv[1]) != 20
```
Input string len should be 20

```c
unsigned long check_password(const char* p) {
    int* ip = (int*)p;
    int i;
	int res=0;
	for(i=0; i<5; i++){
		res += ip[i];
	}
	return res;
}
```

1. char pointer is casted to an int pointer
2. value are read as int values
3. result is compared against hashcode 0x21DD09EC

So split the hashcade into 5 equal parts?
No, cuz 0x21DD09EC%5 = 4!

Split into 4 parts with 0x00 for the last part?
No, cuz input needs to be of len 20. strlen will not count the last 0x00 values.

Last value = 0x01010101
other values = (0x21DD09EC - 0x01010101)/4 = 0x837023a
Last value = Last value + (0x21DD09EC - 0x01010101)%4 = 0x01010104

Input = 0x837023a | 0x837023a | 0x837023a | 0x837023a | 0x01010104

```
$ python -c "import sys; print(sys.byteorder)"
little
```

```
./col `python -c "print('\x3a\x02\x37\x08'*4 + '\x04\x01\x01\x01')"`
```
