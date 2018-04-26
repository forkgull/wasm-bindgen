use super::project;

#[test]
fn simple() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(s: &JsValue);
            }

            #[wasm_bindgen]
            pub fn bar(s: &JsValue) {
                foo(s);
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let ARG: string | null = null;

            export function foo(s: any): void {
                assert.strictEqual(ARG, null);
                ARG = s;
            }

            export function test() {
                assert.strictEqual(ARG, null);
                let sym = (Symbol as any)('test');
                wasm.bar(sym);
                assert.strictEqual(ARG, sym);
            }
        "#)
        .test();
}

#[test]
fn owned() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(s: JsValue);
            }

            #[wasm_bindgen]
            pub fn bar(s: JsValue) {
                foo(s);
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let ARG: any = null;

            export function foo(s: any): void {
                assert.strictEqual(ARG, null);
                ARG = s;
            }

            export function test() {
                assert.strictEqual(ARG, null);
                let sym = (Symbol as any)('test');
                wasm.bar(sym);
                assert.strictEqual(ARG, sym);
            }
        "#)
        .test();
}

#[test]
fn clone() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo1(s: JsValue);
                fn foo2(s: &JsValue);
                fn foo3(s: JsValue);
                fn foo4(s: &JsValue);
                fn foo5(s: JsValue);
            }

            #[wasm_bindgen]
            pub fn bar(s: JsValue) {
                foo1(s.clone());
                foo2(&s);
                foo3(s.clone());
                foo4(&s);
                foo5(s);
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let ARG = (Symbol as any)('test');

            export function foo1(s: any): void { assert.strictEqual(s, ARG); }
            export function foo2(s: any): void { assert.strictEqual(s, ARG); }
            export function foo3(s: any): void { assert.strictEqual(s, ARG); }
            export function foo4(s: any): void { assert.strictEqual(s, ARG); }
            export function foo5(s: any): void { assert.strictEqual(s, ARG); }

            export function test() {
                wasm.bar(ARG);
            }
        "#)
        .test();
}

#[test]
fn promote() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo1(s: &JsValue);
                fn foo2(s: JsValue);
                fn foo3(s: &JsValue);
                fn foo4(s: JsValue);
            }

            #[wasm_bindgen]
            pub fn bar(s: &JsValue) {
                foo1(s);
                foo2(s.clone());
                foo3(s);
                foo4(s.clone());
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let ARG = (Symbol as any)('test');

            export function foo1(s: any): void { assert.strictEqual(s, ARG); }
            export function foo2(s: any): void { assert.strictEqual(s, ARG); }
            export function foo3(s: any): void { assert.strictEqual(s, ARG); }
            export function foo4(s: any): void { assert.strictEqual(s, ARG); }

            export function test() {
                wasm.bar(ARG);
            }
        "#)
        .test();
}

#[test]
fn returning_vector() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo() -> JsValue;
            }

            #[wasm_bindgen]
            pub fn bar() -> Vec<JsValue> {
                let mut res = Vec::new();
                for _ in 0..10 {
                    res.push(foo())
                }
                res
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            export function foo(): any { return { "foo": "bar" }; }

            export function test() {
                const result = wasm.bar();
                assert.strictEqual(result.length, 10);
            }
        "#)
        .test();
}

#[test]
fn another_vector_return() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;


            #[wasm_bindgen]
            pub fn get_array() -> Vec<JsValue> {
                vec![
                    JsValue::from(1),
                    JsValue::from(2),
                    JsValue::from(3),
                    JsValue::from(4),
                    JsValue::from(5),
                    JsValue::from(6),
                ]
            }
        "#)
        .file("test.ts", r#"
            import { get_array } from "./out";
            import * as assert from "assert";

            export function test() {
                assert.deepStrictEqual(get_array(), [1, 2, 3, 4, 5, 6]);
            }
        "#)
        .test();
}
