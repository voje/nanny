run:
	cargo run -- --limit 30 --start "07:00" --end "23:00" --state-path "./state/state.yml" --freq 3 --log-path /tmp/nanny.log

build-win:
	cross build --target i686-pc-windows-gnu

