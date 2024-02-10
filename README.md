
# kvcache

A simple commandline key-value cache, backed by SQLite.

## Usage:

Set a value (in the future, the value will time out after 1 hour):
```
kvcache set key value
```

Get a value:
```
kvcache get key
```

Get a value if we have it, otherwise run a command and store the result:
```
kvcache try key 'command'
```
or
```
kvcache try key "command'"
```

# TODO:
- Avoid hardcoding my own personal home folder path
- Add a time-to-live to the set command
