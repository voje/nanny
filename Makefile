run:
	cargo run -- --limit 30 --start "07:00" --end "23:00" --state-path "./state/state.yml" --freq 3

build-win:
	cross build --target i686-pc-windows-gnu

