use std::process::Command;

use wrapped_mono::{add_internal_call, invokable, jit, metadata::MethodTable, Array, Class, Dim1D, Exception, InteropReceive, Method, Object};

fn main() {
    let current = std::env::current_dir().expect("could not get current working directory");
    let output = Command::new("mcs.bat")
        .arg("-target:library")
        .arg(format!("-out:{}", current.join("examples").join("sample.dll").display()))
        .arg(current.join("examples").join("sample.cs"))
        .output()
        .expect("failed to compile assembly");

    if !output.status.success() {
        panic!("failed to compile assembly: {}{}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        );
    }

    let domain = jit::init("sample_domain", None);
    let asm = domain.assembly_open("examples/sample.dll").expect("failed to load assembly");
    let image = asm.get_image();
    let class = Class::from_name(&image, "", "Sample").expect("could not find class Sample");
    let instance = Object::new(&domain, &class);

    let metadata = MethodTable::from_image(image).expect("could not find metadata for image");
    println!("[{}] Methods", class.num_methods());
    for method in metadata.methods() {
        println!("{}", method.name());
    }

    // Get a constructor method of SomeClass accepting an integer and a string (2 parameters)
    let ctor: Method<(i32,)> = Method::get_from_name(&class, ".ctor", 1).expect("Could not find the constructor!");
    ctor.invoke(Some(instance.clone()), (12,)).expect("Got an exception while calling the Sample class's constructor");

    // Get a method "Count" form SomeClass with 0 parameters returning a number
    let met: Method<()> = Method::get_from_name(&class, "Count", 0).expect("Could not find method \"Count\"!");
    // Call "Count" method on an istance
    let res_obj = met.invoke(Some(instance.clone()), ()).expect("Got an exception while calling Sample::Count").expect("Got null from Count");
    // Unbox the result to get a raw integer from a boxed integer
    let res = res_obj.unbox::<i32>();
    println!("[COUNT] {res}");

    // Create a function with the special "invokable" attribute
    #[invokable]
    fn sqrt(input: f32) -> f32 {
        if input < 0.0{
            // can't get sqrt of a negative number, so create a managed exception and throw it
            unsafe{ Exception::arithmetic().raise() };
        }
        input.sqrt()
    }
    // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" attribute with a rust function
    add_internal_call!("Sample::Sqrt", sqrt);

    // This supports all types with `InteropReceive` trait
    #[invokable]
    fn avg(input: Array<Dim1D, f32>) -> f32 {
        let mut avg = 0.0;
        for i in 0..input.len(){
            let curr = input.get([i]);// get the element at index i
            avg += curr/(input.len() as f32);
        }
        avg
    }

    // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" attribute with a rust function
    add_internal_call!("Sample::Avg", avg);

    let values = Array::from([100.0, 200.0, 20.0, 15.0, 22.0].as_slice());
    let met: Method<(Array<Dim1D, f32>,)> = Method::get_from_name(&class, "Avg", 1).expect("Could not find method \"Avg\"!");
    let res_obj = met.invoke(
        Some(instance.clone()),
        (values,)
    )
        .expect("Got an exception while calling Sample::Avg").expect("Got null from Avg");

    let res = res_obj.unbox::<f32>();
    println!("[AVG([100, 200, 20, 15, 22])] {res}");

    let met: Method<(f32,)> = Method::get_from_name(&class, "Sqrt", 1).expect("Could not find method \"Avg\"!");
    let res_obj = met.invoke(Some(instance.clone()), (res,)).expect("Got an exception while calling Sample::Sqrt").expect("Got null from Avg");
    let sres = res_obj.unbox::<f32>();
    println!("[Sqrt({res})] {sres}");
}
