use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use std::collections::BTreeMap;
use vrl::{
    compiler::{state::RuntimeState, Context, TargetValue, TimeZone},
    value,
    value::{Secrets, Value as VrlValue},
};

pub fn parse_json(c: &mut Criterion) {
    // VRL script: remove the field "age" and add a new field "new_field"
    let src = r#"
        del(.age)
        .new_field = "new value"
    "#;

    let fns = vrl::stdlib::all();

    // compile the program outside the loop
    let result = vrl::compiler::compile(src, &fns).unwrap();
    let mut state = RuntimeState::default();
    let timezone = TimeZone::default();

    c.bench_function("vrl", |b| {
        b.iter(|| {
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

    c.bench_function("native_rust", |b| {
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
