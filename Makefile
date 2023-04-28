run:
	cargo run -- --limit 30 --start "07:00" --end "22:00" --state-path "./state/state.yml"

build-win:
	cross build --target i686-pc-windows-gnu

