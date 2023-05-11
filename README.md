# nanny
Windows parental control with daily allowed hours and daiy usage limit.

See `Makefile` for run commands.

Windows example:
```bash
mkdir C:\Users\me\AppData\Local\nanny\
.\nanny.exe --state-path C:\Users\me\AppData\Local\nanny\state.yml --limit 90 --start "07:00" --end "21:00" --freq 30
```

Install as a Windows service.   
Using a third-party service manager NSSM.   
```bash
# PS1 (Admin)
chocolatey install -y nssm 

$state_path="C:\Users\me\AppData\Local\nanny\state.yml"
$nanny_exe_path="C:\Program Files\nanny\nanny.exe"

nssm install nanny $nanny_exe_path --limit 30 --freq 60 --start "07:00" --end "10:00" --state-path $state_path
nssm start nanny
```

