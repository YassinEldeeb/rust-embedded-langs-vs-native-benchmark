use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rhai::Dynamic;
use rhai::{Engine, Scope};
use serde_json::json;
use std::collections::BTreeMap;
use vrl::{
    compiler::{state::RuntimeState, Context, TargetValue, TimeZone},
    value,
    value::{Secrets, Value as VrlValue},
};

pub fn parse_json(c: &mut Criterion) {
    let engine = Engine::new();
    let rhai_script = r#"
        let map = input;
        map.remove("age");
        map.new_field = "new value";
        map
    "#;
    let compiled_script = engine.compile(rhai_script).unwrap();

    // VRL script: remove the field "age" and add a new field "new_field"
    let src = r#"
        del(.age)
        .new_field = "new value"
    "#;
    // compile the program outside the loop
    let fns = vrl::stdlib::all();

    let result = vrl::compiler::compile(src, &fns).unwrap();
    let mut state = RuntimeState::default();
    let timezone = TimeZone::default();

    c.bench_function("Rhai", |b| {
        b.iter(|| {
            let mut scope = Scope::new();

            let json_str = r#"{"name": "John", "age": 30}"#;
            let map = engine.parse_json(json_str, true).unwrap();

            scope.push_constant("input", Dynamic::from(map));

            let result: Result<rhai::Map, _> =
                engine.eval_ast_with_scope(&mut scope, &compiled_script);
            let modified_map = result.unwrap();

            // Serialize the map back to JSON
            let modified_json = serde_json::to_string(&modified_map).unwrap();

            black_box(modified_json);
        })
    });

    c.bench_function("VRL", |b| {
        b.iter(|| {
            state.clear();
            // define the initial JSON object for VRL
            let mut target = TargetValue {
                value: value!({"name": "John", "age": 30}),
                metadata: VrlValue::Object(BTreeMap::new()),
                secrets: Secrets::default(),
            };

            let mut ctx = Context::new(&mut target, &mut state, &timezone);

            // Execute the VRL program
            result.program.resolve(&mut ctx).unwrap();

            black_box(target.value);
        })
    });

    c.bench_function("Native", |b| {
        b.iter(|| {
            // define the initial JSON object for native Rust
            let mut json_obj = json!({
                "name": "John",
                "age": 30
            });

            // remove the "age" field
            json_obj.as_object_mut().unwrap().remove("age");

            // add a new field "new_field"
            json_obj
                .as_object_mut()
                .unwrap()
                .insert("new_field".to_string(), json!("new value"));

            black_box(json_obj);
        })
    });
}

criterion_group!(benches, parse_json);
criterion_main!(benches);
