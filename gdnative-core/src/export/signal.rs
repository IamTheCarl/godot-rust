use crate::core_types::{GodotString, Variant, VariantType};
use crate::export::{ClassBuilder, ExportInfo, NativeClass, PropertyUsage};

pub struct SignalBuilder<'a, C> {
    class_builder: &'a ClassBuilder<C>,
    name: GodotString,
    args: Vec<SignalArgument>,
}

impl<'a, C: NativeClass> SignalBuilder<'a, C> {
    pub(super) fn new(class_builder: &'a ClassBuilder<C>, name: &str) -> Self {
        Self {
            class_builder,
            name: GodotString::from(name),
            args: vec![],
        }
    }

    /// Add an argument for the signal.
    #[inline]
    pub fn arg(mut self, argument: SignalArgument) -> Self {
        self.args.push(argument);
        self
    }

    /// Finish registering the signal.
    #[inline]
    pub fn done(self) {
        self.class_builder.add_signal(Signal {
            name: self.name,
            args: self.args,
        });
    }
}

pub(crate) struct Signal {
    pub name: GodotString,
    pub args: Vec<SignalArgument>,
}

pub struct SignalArgument {
    pub name: GodotString,
    pub default: Variant,
    pub export_info: ExportInfo,
    pub usage: PropertyUsage,
}

/// Argument (or parameter) in a signal declaration.
/// You can use one of the constructors for convenience, or provide each field separately.
impl SignalArgument {
    /// The most common way of constructing a signal argument -- name and type.
    #[inline]
    pub fn new(name: impl Into<GodotString>, argument_type: VariantType) -> Self {
        Self {
            name: name.into(),
            default: Variant::nil(),
            export_info: ExportInfo::new(argument_type),
            usage: PropertyUsage::DEFAULT,
        }
    }

    /// Construct a signal argument that accepts any dynamic type.
    #[inline]
    pub fn untyped(name: impl Into<GodotString>) -> Self {
        Self {
            name: name.into(),
            default: Variant::nil(),
            export_info: ExportInfo::new(VariantType::Nil),
            usage: PropertyUsage::DEFAULT,
        }
    }

    /// Construct from name and default value. The type is deduced from the default.
    #[inline]
    pub fn with_default(name: impl Into<GodotString>, default_value: Variant) -> Self {
        let variant_type = default_value.get_type();

        Self {
            name: name.into(),
            default: default_value,
            export_info: ExportInfo::new(variant_type),
            usage: PropertyUsage::DEFAULT,
        }
    }
}
