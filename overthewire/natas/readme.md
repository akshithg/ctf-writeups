# Overthewire | Natas

## level 0

```txt
inspect page source for flag (as a comment)
```

## level 1

```txt
inspect page source for flag (as a comment)
use keyboard shortcut instead of mouse for console
```

## level 2

```txt
inspect
image link
explore directory
open /files/users.txt
```

## level 3

```txt
inspect
not even google - robots.txt
explore dir on robots.txt
```

## level 4

```txt
refresh link on page
edit request to manually change the referrer header
```

## level 5

```txt
change cookie loggedin to 1
```

## level 6

```txt
view source
check include "includes/secret.inc" file
secret as comment
send secret
```

## level 7

```txt
view source
use the password file as the page parameter
```

## level 8

```txt
view source
reverse encodedSecret
bin2hex(strrev(base64_encode($secret)));
```

## level 9

```txt
view source
$key is used to run a shell command
and passthrough the raw output
inject cat /etc/natas_webpass/natas10 via $key
```

## level 10

```txt
view source
cannot use ;|& in the injected command
use grep wildcard to match all lines in a file
key = [a-z,A-Z,0-9]} /etc/natas_webpass/natas11
```
