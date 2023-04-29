# nanny
Windows parental control with daily allowed hours and daiy usage limit.

See `Makefile` for run commands.

Windows example:
```bash
mkdir C:\Users\me\AppData\Local\nanny\
.\nanny.exe --state-path C:\Users\me\AppData\Local\nanny\state.yml --limit 90 --start "07:00" --end "21:00" --freq 30
```

