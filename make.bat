if %1 == run (
	cargo run
) else if %1 == teste (
	rustc src/teste.rs 
	teste.exe
)
