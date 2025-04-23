use deno_core::*;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use tokio::runtime::Runtime;

#[op2(fast)]
fn op_godot_print(#[string] text: String) {
    println!("op_godot_print: {}", text);
}

/// An async function that creates a JsRuntime, executes a script,
/// and then drops the runtime when finished.
async fn execute_script_async() {
    let thread_id = std::thread::current().id();
    println!("Starting script on thread {:?}", thread_id);

    // Build an extension with our op.
    const OPS: &[OpDecl] = &[op_godot_print()];
    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(OPS),
        ..Default::default()
    };

    // Create a new JsRuntime instance.
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    // Build a script that calls our op.
    let script = format!(
        r#"Deno.core.ops.op_godot_print("Hello world from {:?}");"#,
        thread_id
    );

    // Execute the script. When this async function returns,
    // js_runtime goes out of scope and is dropped.
    js_runtime.execute_script("<usage>", script).unwrap();
    println!("Script executed on thread {:?}", thread_id);
}

/// This async function spawns 10 tasks and awaits them in reverse order
/// by using FuturesOrdered and push_front(), so that the drop (and thus
/// the Exit() call on the V8 isolate) happens in LIFO order.
async fn async_main() -> String {
    let mut futures = FuturesOrdered::new();

    // Instead of pushing in natural order, we push each future at the front.
    for i in 0..10 {
         println!("Spawning task {}", i);
         futures.push_front(execute_script_async());
    }

    // Await all futures sequentially.
    while let Some(_) = futures.next().await {
        // Nothing to do here.
    }
    "All scripts executed.".into()
}

/// Create a Tokio runtime, run our async_main, and return immediately.
pub fn runtime_main() -> String {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let result = rt.block_on(async_main());
    println!("{}", result);
    "Runtime started".into()
}
