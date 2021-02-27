# Engine for making key-value queries in csv file

## Example

```rust
let opts = EngineOptions::default();
let mut engine: Engine<String, YourType> = Engine::from_file_with_opts("file.csv", YourType::make_key, opts).unwrap();
let value = engine.get_cached(&"key".to_string());
println!("{:?}", value);
```
