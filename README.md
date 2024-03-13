Convert env vars to toml text

## Syntax
`__` split to `.`

```
APP_TITLE="TOML Example"
APP_OWNER__NAME="Tom Preston-Werner"
APP_OWNER__DOB=1979-05-27T07:32:00-08:00
APP_DATABASE__ENABLED=true
APP_DATABASE__PORTS=[ 8000, 8001, 8002 ]
APP_DATABASE__DATA=[ ["delta", "phi"], [3.14] ]
APP_DATABASE__TEMP_TARGETS={ cpu = 79.5, case = 72.0 }
APP_SERVERS__ALPHA__IP="10.0.0.1"
APP_SERVERS__ALPHA__FRONT="frontend"


```
PRIFIX: `APP_`

RESULT：
```toml
title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00

[database]
enabled = true
ports = [ 8000, 8001, 8002 ]
data = [ ["delta", "phi"], [3.14] ]
temp_targets = { cpu = 79.5, case = 72.0 }

[servers.alpha]
ip = "10.0.0.1"
role = "frontend"

```
