use wasmer::{imports, Function, Instance, Module, Store};

const PATH: &str = todo!();
const PUBLIC_INPUT: &'static [u64] = &[todo!()];
const PRIVATE_INPUT: &'static [u64] = &[todo!()];

fn require(arg: i32) {
    assert!(arg != 0);
}

fn wasm_input(arg: i32) -> u64 {
    let mut public_input = PUBLIC_INPUT.clone().to_vec();
    let mut private_input = PRIVATE_INPUT.clone().to_vec();

    if arg == 0 {
        public_input.pop().unwrap()
    } else if arg == 1 {
        private_input.pop().unwrap()
    } else {
        panic!()
    }
}

fn main() -> anyhow::Result<()> {
    let mut store = Store::default();
    let module_wasm = std::fs::read(PATH).unwrap();
    let module = Module::new(&store, &module_wasm)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {
        "env" => {
            "require" => Function::new_typed(&mut store, require),
            "wasm_input" => Function::new_typed(&mut store, wasm_input),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let zkmain = instance.exports.get_function("zkmain")?;
    let result = zkmain.call(&mut store, &[])?;

    println!("{:?}", result);

    Ok(())
}
