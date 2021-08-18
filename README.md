# clini
clini is a simple tool to do basic modification of ini files

## Getting Started

- Get a value
```
clini get --section section --key key ./example/sample.ini` 
```

- Set a value
```
clini set --section new_section --key foo --value bar ./example/sample.ini
```

- Delete a value
```
clini del --section new_section --key foo ./example/sample.ini
```

## Project Inspiration
- <https://www.horstmuc.de/wbat32.htm>
- <https://github.com/pixelb/crudini>
- <https://en.wikipedia.org/wiki/INI_file>
- <https://github.com/zonyitoo/rust-ini>
- <https://github.com/mexili/configparser-rs>

