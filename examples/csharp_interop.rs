use std::process::Command;

use wrapped_mono::*;

/*
C# code in AsmWithVec3
using System.Runtime.CompilerServices;
namespace Rasl {
    public struct Vec3{
        float x;
        float y;
        float z;

        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        public extern void DoMagic();
    }
}
*/

//this types layout does not differ on managed and unmanaged side.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
} 

impl Vec3 {
    pub extern "C" fn _invokable_do_vec3_magic(
        input: <Vec3 as wrapped_mono::InteropReceive>::SourceType,
    ) {
        let input = <Vec3>::get_rust_rep(input);
        Self::do_vec3_magic(input);
        //unsafe { fnc_call_res_val.return_value_to_mono() }
    }

    pub fn do_vec3_magic(self) {
        println!("Doing magic with vec3!");
    }

    pub fn expose() {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        type FnTypeDoVec3Magic = extern "C" fn(<Vec3 as InteropReceive>::SourceType);

        {
            let cstr = std::ffi::CString::new("Rasl.Vec3::DoMagic").expect("Could note create cstring");
            let fnc_ptr = Vec3::_invokable_do_vec3_magic as FnTypeDoVec3Magic as *const core::ffi::c_void;
            unsafe{ wrapped_mono::binds::mono_add_internal_call(cstr.as_ptr(), fnc_ptr) };
            drop(cstr);
        }
    }
}

unsafe impl InteropSend for Vec3 {}
impl InteropBox for Vec3 {}

impl InteropReceive for Vec3 {
    type SourceType = (
        <f32 as InteropReceive>::SourceType,
        <f32 as InteropReceive>::SourceType,
        <f32 as InteropReceive>::SourceType,
    );
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        Self{ x: mono_arg.0, y: mono_arg.1, z: mono_arg.2 }
    }
}

use lazy_static::lazy_static;
lazy_static!{
    static ref Class_Vec3: Class = {
        let img = Assembly::assembly_loaded("Rasl").expect("Could not find assembly").get_image();
        Class::from_name(&img,"Rasl","Vec3").expect("Could not find Rasl.Vec3!")
    };
}
impl InteropClass for Vec3 {
    fn get_mono_class()-> Class { *Class_Vec3 }
}

lazy_static!{
    static ref Class_Transform: Class = {
        let img = Assembly::assembly_loaded("Rasl").expect("Could not find assembly").get_image();
        Class::from_name(&img, "Rasl", "Transform").expect("Could not find Rasl.Transform!")
    };
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}
impl InteropClass for Transform {
    fn get_mono_class() -> Class { *Class_Transform }
}
impl InteropReceive for Transform {
    type SourceType = (
        <Vec3 as InteropReceive>::SourceType,
        <Vec3 as InteropReceive>::SourceType,
        <Vec3 as InteropReceive>::SourceType,
    );
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        Self{
            position: Vec3::get_rust_rep(mono_arg.0),
            rotation: Vec3::get_rust_rep(mono_arg.1),
            scale: Vec3::get_rust_rep(mono_arg.2),
        }
    }
}

lazy_static!{
    static ref Class_Component: Class = {
        let img = Assembly::assembly_loaded("Rasl").expect("Could not find assembly").get_image();
        Class::from_name(&img, "Rasl", "Component").expect("Could not find Rasl.Component!")
    };
}

lazy_static!{
    static ref Class_Entity: Class = {
        let img = Assembly::assembly_loaded("Rasl").expect("Could not find assembly").get_image();
        Class::from_name(&img, "Rasl", "Entity").expect("Could not find Rasl.Entity!")
    };
}

lazy_static!{
    static ref Class_Script: Class = {
        let img = Assembly::assembly_loaded("Rasl").expect("Could not find assembly").get_image();
        Class::from_name(&img, "Rasl", "Script").expect("Could not find Rasl.Script!")
    };
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct Entity {
    entity_id: u32
}
impl InteropClass for Entity {
    fn get_mono_class() -> Class { *Class_Entity }
}
impl InteropReceive for Entity {
    type SourceType = (u32,);
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        Self { entity_id: mono_arg.0 }
    }
}
impl InteropBox for Entity {}
unsafe impl InteropSend for Entity {}

impl Entity {
    pub extern "C" fn get_transform_invokable(this: <Entity as InteropReceive>::SourceType) -> Object {
        let entity = Entity::get_rust_rep(this);
        let domain = Domain::get_current().unwrap();
        let transform = entity.get_transform();
        let is_component = Class_Vec3.get_interfaces().iter().any(|i| i.get_name().as_str() == "Component");

        let position = Object::new(&domain, &*Class_Vec3);
        let invokable: Method<(f32, f32, f32)> = Method::get_from_name(&*Class_Vec3, ".ctor", 3).expect("could not find Vec3 constructor");
        invokable.invoke(Some(position.clone()), (transform.position.x, transform.position.y, transform.position.z));
        if is_component {
            let invokable: Method<(u32,)> = Method::get_from_name(&*Class_Component, ".ctor", 1).expect("could not find Component constructor");
            invokable.invoke(Some(position.clone()), (entity.entity_id,));
        }

        let rotation = Object::new(&domain, &*Class_Vec3);
        let invokable: Method<(f32, f32, f32)> = Method::get_from_name(&*Class_Vec3, ".ctor", 3).expect("could not find Vec3 constructor");
        invokable.invoke(Some(rotation.clone()), (transform.position.x, transform.position.y, transform.position.z));
        if is_component {
            let invokable: Method<(u32,)> = Method::get_from_name(&*Class_Component, ".ctor", 1).expect("could not find Component constructor");
            invokable.invoke(Some(rotation.clone()), (entity.entity_id,));
        }

        let scale = Object::new(&domain, &*Class_Vec3);
        let invokable: Method<(f32, f32, f32)> = Method::get_from_name(&*Class_Vec3, ".ctor", 3).expect("could not find Vec3 constructor");
        invokable.invoke(Some(scale.clone()), (transform.position.x, transform.position.y, transform.position.z));
        if is_component {
            let invokable: Method<(u32,)> = Method::get_from_name(&*Class_Component, ".ctor", 1).expect("could not find Component constructor");
            invokable.invoke(Some(scale.clone()), (entity.entity_id,));
        }

        let result = Object::new(&domain, &*Class_Transform);
        let invokable: Method<(Object, Object, Object)> = Method::get_from_name(&*Class_Transform, ".ctor", 3).expect("could not find Transform constructor");
        invokable.invoke(Some(scale.clone()), (position, rotation, scale));
        if is_component {
            let invokable: Method<(u32,)> = Method::get_from_name(&*Class_Component, ".ctor", 1).expect("could not find Component constructor");
            invokable.invoke(Some(result.clone()), (entity.entity_id,));
        }

        result
    }

    pub fn get_transform(self) -> Transform {
        Transform::default()
    }
}

fn main() {
    let current = std::env::current_dir().expect("could not get current working directory");
    let output = Command::new("mcs.bat")
        .arg("-target:library")
        .arg(format!("-out:{}", current.join("examples").join("Rasl.dll").display()))
        .arg(current.join("examples").join("rasl.cs"))
        .output()
        .expect("failed to compile assembly");

    if !output.status.success() {
        panic!("failed to compile assembly: {}{}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        );
    }

    let output = Command::new("mcs.bat")
        .arg("-target:library")
        .arg(format!("-out:{}", current.join("examples").join("Script.dll").display()))
        .arg(current.join("examples").join("Rasl.cs"))
        .arg(current.join("examples").join("script.cs"))
        .output()
        .expect("failed to compile assembly");

    if !output.status.success() {
        panic!("failed to compile assembly: {}{}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        );
    }

    let domain = jit::init("rasl_domain", None);
    domain.assembly_open("examples/Rasl.dll").expect("could not load Rasl.dll");
    {
        let cstr = std::ffi::CString::new("Rasl.Entity::GetTransform").expect("Could note create cstring");
        let fnc_ptr = Entity::get_transform_invokable as extern "C" fn(<Entity as InteropReceive>::SourceType) -> Object as *const core::ffi::c_void;
        unsafe{ wrapped_mono::binds::mono_add_internal_call(cstr.as_ptr(), fnc_ptr) };
        drop(cstr);
    }

    let script_asm = domain.assembly_open("examples/Script.dll").expect("could not load Script.dll");

    let image = script_asm.get_image();
    let class = Class::from_name(&image, "", "CustomScript").expect("could not find class CustomScript");

    let instance = Object::new(&domain, &class);

    let ctor: Method<()> = Method::get_from_name(&class, ".ctor", 0).expect("Could not find CustomScript::.ctor!");
    ctor.invoke(Some(instance.clone()), ()).expect("Got an exception while calling .ctor");

    let ctor: Method<(u32,)> = Method::get_from_name(&Class_Entity, ".ctor", 1).expect("Could not find Entity::.ctor!");
    ctor.invoke(Some(instance.clone()), (4,)).expect("Got an exception while calling .ctor");

    let ctor: Method<()> = Method::get_from_name(&class, "Update", 0).expect("Could not find Script!");
    ctor.invoke(Some(instance.clone()), ()).expect("Got an exception while calling Update");

    //let value = class.get_field("x").unwrap().get_value::<f32>(&instance).expect("failed to extraxt field `x`");
    //println!("[X] {value}");
    //let value = class.get_field("y").unwrap().get_value::<f32>(&instance).expect("failed to extraxt field `y`");
    //println!("[Y] {value}");
    //let value = class.get_field("z").unwrap().get_value::<f32>(&instance).expect("failed to extraxt field `z`");
    //println!("[Z] {value}");

    //let array = make_vec3_array(&domain);
    //println!("[Array<Vec3>] {}", array.len());
    //let result = box_n_unbox_vec3(&domain, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
    //println!("[Box::Vec3] {:?}", result);
}
