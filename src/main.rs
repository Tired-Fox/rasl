#[derive(Default, Clone)]
struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl rhai::CustomType for Rectangle {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("Rectangle")
            .with_get_set(
                "x",
                |obj: &mut Self| obj.x as i64,
                |obj: &mut Self, val: i64| obj.x = val as usize,
            )
            .with_get_set(
                "y",
                |obj: &mut Self| obj.y as i64,
                |obj: &mut Self, val: i64| obj.y = val as usize,
            )
            .with_get_set(
                "width",
                |obj: &mut Self| obj.width as i64,
                |obj: &mut Self, val: i64| obj.width = val as usize,
            )
            .with_get_set(
                "height",
                |obj: &mut Self| obj.height as i64,
                |obj: &mut Self, val: i64| obj.height = val as usize,
            )
            .with_fn("area", |this: Rectangle| {
                (this.height * this.width) as i64
            })
            .with_fn("diagonal", |this: Rectangle| {
                (this.height.pow(2) as f64 + this.width.pow(2) as f64).sqrt()
            });
    }
}

impl mlua::UserData for Rectangle {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_set("x", |_, this, val| {
            this.x = val;
            Ok(())
        });
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_set("y", |_, this, val| {
            this.y = val;
            Ok(())
        });
        fields.add_field_method_get("width", |_, this| Ok(this.width));
        fields.add_field_method_set("width", |_, this, val| {
            this.width = val;
            Ok(())
        });
        fields.add_field_method_get("height", |_, this| Ok(this.height));
        fields.add_field_method_set("height", |_, this, val| {
            this.height = val;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("area", |_, this, ()| Ok(this.height * this.width));
        methods.add_method("diagonal", |_, this, ()| {
            Ok((this.height.pow(2) as f64 + this.width.pow(2) as f64).sqrt())
        });

        // Constructor
        methods.add_meta_function(mlua::MetaMethod::Call, |_, ()| Ok(Rectangle::default()));
    }
}


fn main() {
    let mut engine = rhai::Engine::new();
    engine.build_type::<Rectangle>();
    engine.register_fn("new_rect", Rectangle::default);

    let lua = mlua::Lua::new();

    let rectangle = Rectangle::default();
    _ = lua.load(mlua::chunk! {
        local rect = $rectangle()
        rect.width = 10
        rect.height = 5
        print(rect:area())
        print(rect:diagonal())
    })
        .exec()
        .expect("failed to evaluate lua script");

    _ = engine.eval::<()>(
        "
            let rect = new_rect();
            rect.width = 10;
            rect.height = 5;
            print(rect.area());
            print(rect.diagonal());
        ",
    )
        .expect("failed to evaluate rhai script");
}
