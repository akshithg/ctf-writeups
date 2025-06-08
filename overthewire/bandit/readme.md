# Overthewire | Bandit

## level 0

```sh
cat readme
```

## level 1

```sh
cat ./-
```

## level 2

```sh
cat spaces\ in\ this\ filename
```

## level 3

```sh
cat inhere/.hidden
```

## level 4

```sh
cd inhere; file ./*
cat ./-file07
```

## level 5

```sh
find . -size 1033c
cat ./inhere/maybehere07/.file2
```

## level 6

```sh
find / -user bandit7 -group bandit6
cat /var/lib/dpkg/info/bandit7.password
```

## level 7

```sh
cat data.txt | grep millionth
```

## level 8

```sh
cat data.txt | sort | uniq -u
```

## level 9

```sh
strings data.txt  | grep ====
```

## level 10

```sh
cat data.txt | base64 --decode
```

## level 11

```sh
cat data.txt | tr 'A-Za-z' 'N-ZA-Mn-za-m'
```

## level 12

```sh
xxd -r data.txt > /tmp/nanana/data; cd /tmp/nanana
zcat data | bzcat | zcat | tar xO | tar xO | bzcat | tar xO | zcat
```

## level 13

```sh
ssh -i sshkey.private bandit14@localhost
```

## level 14

```sh
nc localhost 30000 < /etc/bandit_pass/bandit14
```

## level 15

```sh
openssl s_client -host localhost -port 30001 -ign_eof < /etc/bandit_pass/bandit15
```

## level 16

```sh
nmap -p 31000-32000 localhost
openssl s_client -host localhost -ign_eof -port 31790 < /etc/bandit_pass/bandit16
touch /tmp/ababab/ssh.key
copy private key to this file
chmod 400 /tmp/ababab/ssh.key
ssh -i /tmp/ababab/ssh.key bandit17@localhost
```

## level 17

```sh
diff passwords.old passwords.new | grep ">"
```

## level 18

```sh
scp -P 2220 bandit18@bandit.labs.overthewire.org:/home/bandit18/readme ./
cat readme
```

## level 19

```sh
./bandit20-do cat /etc/bandit_pass/bandit2
```

## level 20

```sh
nc -l 33333 < /etc/bandit_pass/bandit20 &
./suconnect 33333
```

## level 21

```sh
cat /etc/cron.d/cronjob_bandit22
cat /usr/bin/cronjob_bandit22.sh
cat /tmp/t7O6lds9S0RqQh9aMcz6ShpAoZKF7fgv
```

## level 22

```sh
cat /etc/cron.d/cronjob_bandit23
cat /usr/bin/cronjob_bandit23.sh
cat /tmp/`echo I am user bandit23 | md5sum | cut -d ' ' -f 1`
```

## level 23

```sh
cat /etc/cron.d/cronjob_bandit24
cat /usr/bin/cronjob_bandit24.sh
mkdir /tmp/bcbcbc
chmod 777 /tmp/bcbcbc
touch /tmp/bcbcbc/getpass.sh
chmod 777 /tmp/bcbcbc/getpass.sh
cat /tmp/bcbcbc/getpass.sh
    #!/bin/sh
    cat /etc/bandit_pass/bandit24 > /tmp/bcbcbc/pass
cp /tmp/bcbcbc/getpass.sh /var/spool/bandit24
cat /tmp/bcbcbc/pass
```

## level 24

```sh
bandit24 = `cat /etc/bandit_pass/bandit24`
for i in {1000..9999}; do echo $bandit24 $i | nc localhost 30002 >> results & done
cat results | sort | uniq
```

## level 25

```sh
ssh -i bandit26.sshkey bandit26@localhost # in a tiny window
v # launcehes vim
:set shell=/bin/bash
:shell
cat /etc/bandit_pass/bandit26
```

## level 26

```sh
./bandit27-do cat /etc/bandit_pass/bandit27
```

## level 27

```sh
mkdir /tmp/27git && cd /tmp/27git
git clone ssh://bandit27-git@localhost/home/bandit27-git/repo
cat repo/README
```

## level 28

```sh
mkdir /tmp/28git && cd /tmp/28git
git clone ssh://bandit28-git@localhost/home/bandit28-git/repo
cd repo
git log
git checkout 196c3edc79e362fe89e0d75cfeef079d8c67beef
cat README.md
```

## level 29

```sh
mkdir /tmp/29git && cd /tmp/29git
git clone ssh://bandit29-git@localhost/home/bandit29-git/repo
cd repo
git branch -a
git checkout dev
cat README.md
```

## level 30

```sh
mkdir /tmp/30git && cd /tmp/30git
git clone ssh://bandit30-git@localhost/home/bandit30-git/repo
cd repo
git tag -l
git show secret
```

## level 31

```sh
mkdir /tmp/31git && cd /tmp/31git
git clone ssh://bandit31-git@localhost/home/bandit31-git/repo
cd repo
touch key.txt
echo "May I come in?" > key.txt
cat .gitignore
git add -f key.txt
git commit -m 'yo'
git push
```

## level 32

```sh
$0
cat /etc/bandit_pass/bandit33
```

## level 33

```sh
💯
```
